#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::{self};
use embassy_time::{Duration, Timer};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::mutex::Mutex;
use embedded_graphics::{pixelcolor::Rgb888, prelude::*, draw_target::DrawTarget, geometry::Point, Pixel};
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

// Module declarations
mod game;
mod colors;
mod animator;

use game::GameBoard;
use colors::ColorState;
use animator::{CellAnimator, apply_brightness};

// Static storage for shared state
static GAME_BOARD: StaticCell<Mutex<CriticalSectionRawMutex, GameBoard>> = StaticCell::new();
static COLOR_STATE: StaticCell<Mutex<CriticalSectionRawMutex, ColorState>> = StaticCell::new();
static ANIMATOR: StaticCell<Mutex<CriticalSectionRawMutex, CellAnimator>> = StaticCell::new();

// Global references once initialized
static mut GAME_BOARD_REF: Option<&'static Mutex<CriticalSectionRawMutex, GameBoard>> = None;
static mut COLOR_STATE_REF: Option<&'static Mutex<CriticalSectionRawMutex, ColorState>> = None;
static mut ANIMATOR_REF: Option<&'static Mutex<CriticalSectionRawMutex, CellAnimator>> = None;
static mut HUB75_REF: Option<&'static Mutex<CriticalSectionRawMutex, DummyHub75>> = None;

// Dummy HUB75 driver for compilation - will be replaced with actual hub75-pio driver
struct DummyHub75 {
    width: u32,
    height: u32,
}

impl DummyHub75 {
    fn new(_width: u32, _height: u32) -> Self {
        Self { width: _width, height: _height }
    }
}

impl DrawTarget for DummyHub75 {
    type Color = Rgb888;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        // In real implementation, this would write to the HUB75 driver
        for _pixel in pixels {
            // Write pixel to framebuffer
        }
        Ok(())
    }
}

impl OriginDimensions for DummyHub75 {
    fn size(&self) -> embedded_graphics::geometry::Size {
        embedded_graphics::geometry::Size::new(self.width, self.height)
    }
}

static HUB75: StaticCell<Mutex<CriticalSectionRawMutex, DummyHub75>> = StaticCell::new();

/// Task 1: Game Update - runs every 500ms
#[embassy_executor::task]
async fn game_update_task() {
    loop {
        Timer::after(Duration::from_millis(500)).await;
        
        unsafe {
            if let Some(board_ref) = GAME_BOARD_REF {
                let mut board = board_ref.lock().await;
                let changes = board.update_generation();
                
                // Pass changes to animator
                if let Some(animator_ref) = ANIMATOR_REF {
                    let mut animator = animator_ref.lock().await;
                    animator.start_transition(&changes);
                }
                
                info!("Game generation updated, {} cells changed", changes.len());
            }
        }
    }
}

/// Task 2: Color Cycle - runs every 5 seconds
#[embassy_executor::task]
async fn color_cycle_task() {
    loop {
        Timer::after(Duration::from_millis(5000)).await;
        
        unsafe {
            if let Some(color_ref) = COLOR_STATE_REF {
                let mut color_state = color_ref.lock().await;
                color_state.advance();
                info!("Color advanced to index {}", color_state.current_idx);
            }
        }
    }
}

/// Task 3: Render - runs at 60 FPS (~16ms per frame)
#[embassy_executor::task]
async fn render_task() {
    loop {
        Timer::after(Duration::from_millis(16)).await;
        
        unsafe {
            // Update animator progress
            if let Some(animator_ref) = ANIMATOR_REF {
                let mut animator = animator_ref.lock().await;
                animator.update(16);
            }
            
            // Get current color
            let current_color = if let Some(color_ref) = COLOR_STATE_REF {
                let color_state = color_ref.lock().await;
                color_state.get_color()
            } else {
                Rgb888::new(255, 0, 255) // Fallback magenta
            };
            
            // Render frame
            if let (Some(board_ref), Some(animator_ref), Some(hub75_ref)) = 
                (GAME_BOARD_REF, ANIMATOR_REF, HUB75_REF) {
                let board = board_ref.lock().await;
                let animator = animator_ref.lock().await;
                let mut hub75 = hub75_ref.lock().await;
                
                // Draw each cell
                for y in 0..64 {
                    for x in 0..64 {
                        let is_alive = board.get_cell(x, y);
                        let brightness = animator.get_cell_brightness(x, y, is_alive);
                        let pixel_color = apply_brightness(current_color, brightness);
                        
                        // Draw pixel
                        let _ = hub75.draw_iter(core::iter::once(Pixel(Point::new(x as i32, y as i32), pixel_color)));
                    }
                }
                
                // In real implementation, would call hub75.flush() or similar
            }
        }
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("Starting LED Matrix Game of Life");
    
    // Initialize peripherals
    let _p = embassy_rp::init(Default::default());
    
    // Initialize HUB75 driver (dummy for now)
    // In real implementation:
    // let pio = pio::Pio::new(_p.PIO0, _p.DMA_CH0);
    // Configure pins and create Hub75 driver
    let hub75 = DummyHub75::new(64, 64);
    let hub75_ref = HUB75.init(Mutex::new(hub75));
    
    // Initialize game board with random state
    let mut board = GameBoard::new();
    board.random_init(12345); // Use fixed seed for reproducibility
    let board_ref = GAME_BOARD.init(Mutex::new(board));
    info!("Game board initialized with random state");
    
    // Initialize color state
    let color_state = ColorState::new();
    let color_ref = COLOR_STATE.init(Mutex::new(color_state));
    info!("Color state initialized");
    
    // Initialize animator
    let animator = CellAnimator::new();
    let animator_ref = ANIMATOR.init(Mutex::new(animator));
    info!("Animator initialized");
    
    // Store global references
    unsafe {
        GAME_BOARD_REF = Some(board_ref);
        COLOR_STATE_REF = Some(color_ref);
        ANIMATOR_REF = Some(animator_ref);
        HUB75_REF = Some(hub75_ref);
    }
    
    // Spawn all tasks
    spawner.spawn(game_update_task()).unwrap();
    spawner.spawn(color_cycle_task()).unwrap();
    spawner.spawn(render_task()).unwrap();
    
    info!("All tasks spawned, entering main loop");
    
    // Keep main alive
    loop {
        Timer::after(Duration::from_secs(60)).await;
        info!("Main loop tick - system running");
    }
}
