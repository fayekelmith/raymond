#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use hub75_pio;
use hub75_pio::dma::DMAExt;
use hub75_pio::lut::GammaLut;

use defmt::*;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::mutex::Mutex;
use embedded_graphics::pixelcolor::Rgb888;
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};


use rp_pico as bsp;
use rp2040_hal::pio::PIOExt;
use rp2040_hal::{
    clocks::{init_clocks_and_plls, Clock},
    pac,
    sio::Sio,
    watchdog::Watchdog,
};

use embassy_time_driver::Driver;
use core::task::Waker;

struct Rp2040TimeDriver;

embassy_time_driver::time_driver_impl!(static DRIVER: Rp2040TimeDriver = Rp2040TimeDriver);

impl Driver for Rp2040TimeDriver {
    fn now(&self) -> u64 {
        // Read the RP2040 TIMER peripheral (runs at 1 MHz)
        let timer = unsafe { &*pac::TIMER::ptr() };
        let lo = timer.timerawl.read().bits() as u64;
        let hi = timer.timerawh.read().bits() as u64;
        (hi << 32) | lo
    }

    fn schedule_wake(&self, _at: u64, waker: &Waker) {
        waker.wake_by_ref();
    }
}

// Module declarations
mod game;
mod colors;
mod animator;

use game::GameBoard;
use colors::ColorState;
use animator::{CellAnimator, apply_brightness};

type Hub75Display = hub75_pio::Display<
'static, hub75_pio::dma::CH1, 64, 64, 12, Rgb888, 5>;


static mut DISPLAY_BUFFER: hub75_pio::DisplayMemory<64, 64, 12> = hub75_pio::DisplayMemory::new();

// Static storage for shared state
static GAME_BOARD: StaticCell<Mutex<CriticalSectionRawMutex, GameBoard>> = StaticCell::new();
static COLOR_STATE: StaticCell<Mutex<CriticalSectionRawMutex, ColorState>> = StaticCell::new();
static ANIMATOR: StaticCell<Mutex<CriticalSectionRawMutex, CellAnimator>> = StaticCell::new();
static HUB75: StaticCell<Mutex<CriticalSectionRawMutex, Hub75Display>> = StaticCell::new();

// Global references once initialized
static mut GAME_BOARD_REF: Option<&'static Mutex<CriticalSectionRawMutex, GameBoard>> = None;
static mut COLOR_STATE_REF: Option<&'static Mutex<CriticalSectionRawMutex, ColorState>> = None;
static mut ANIMATOR_REF: Option<&'static Mutex<CriticalSectionRawMutex, CellAnimator>> = None;
static mut HUB75_REF: Option<&'static Mutex<CriticalSectionRawMutex, Hub75Display>> = None;
static mut LUT: Option<GammaLut<12, Rgb888, hub75_pio::lut::Init>> = None;

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

/// Task 2: Color Cycle - runs every 250ms
#[embassy_executor::task]
async fn color_cycle_task() {
    loop {
        Timer::after(Duration::from_millis(250)).await;
        
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
                        
                        let _ = hub75.set_pixel(x as usize, y as usize, pixel_color);
                    }
                }
                let _ = hub75.commit();
            }
        }
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("Starting LED Matrix Game of Life");
    
    // Take PAC peripherals: owning the hardware resources in software
    let mut pac = pac::Peripherals::take().unwrap();
    let _core = pac::CorePeripherals::take().unwrap();


    // initializing the clocks
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let external_xtal_freq_hz = 12_000_000u32;
    let clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();
    
    info!("Clocks initialized: System @ {} Hz", clocks.system_clock.freq().to_Hz());
    
    // Initialize single-cycle I/O (needed for GPIO)
    let sio = Sio::new(pac.SIO);
    
    // Get GPIO pins using Board Support Package (nice pin names!)
    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );
    
    info!("Hardware initialization complete, time driver ready");

    //=======
    /*
    1. PIO (Programmable I/O) - Split into 3 state machines
   ├─ SM0: Clocks out pixel data -- Data needs to flow out (RGB Values)
   ├─ SM1: Controls row addressing  -- Rows need to be selected (A-E)
   └─ SM2: Manages output enable (brightness) -- OE needs to be toggled for brightness control

2. DMA (Direct Memory Access) - Split into 4 channels
   ├─ CH0: Framebuffer → PIO -- Main data flow: framebuffer in RAM → PIO for output
   ├─ CH1: Loop back to CH0  -- After one full frame, trigger next transfer for continuous output
   ├─ CH2: Delay values → PIO -- For brightness control: timing of OE signal to achieve different brightness levels
   └─ CH3: Loop back to CH2 -- After one brightness cycle, trigger next for continuous brightness control

3. GPIO Pins - Convert to right types (14 pins total) 
   ├─ 6 for RGB data (R1,G1,B1,R2,G2,B2)
   ├─ 5 for row addressing (A,B,C,D,E)
   └─ 3 for control signals (CLK, LAT/STB, OE)

4. GammaLut - Color correction lookup table 
   └─ Precompute a gamma correction LUT to map 8-bit brightness values to corrected values for better visual quality

5. Display - Put it all together!
     */

    let (mut pio, sm0, sm1, sm2, _) = pac.PIO0.split(&mut pac.RESETS);

    //reset DMA controller to ensure it's in a known state before configuring channels
    let resets = &mut pac.RESETS;
    resets.reset.modify(|_, w| w.dma().set_bit()); // turn off the DMA, wipes state
    resets.reset.modify(|_, w| w.dma().clear_bit()); // power back on
    while resets.reset_done.read().dma().bit_is_clear() {} // wait till fully botted and ready
    info!("DMA controller reset and ready");

    // Split DMA into 4 channels
    let dma = pac.DMA.split();

    unsafe {
        LUT = Some(GammaLut::new().init((1.0, 1.0, 1.0)));
    }

     let mut display = unsafe {
        hub75_pio::Display::new(
            &mut DISPLAY_BUFFER,
            hub75_pio::DisplayPins {
                r1: pins.gpio0.into_function().into_pull_type().into_dyn_pin(),
                g1: pins.gpio1.into_function().into_pull_type().into_dyn_pin(),
                b1: pins.gpio2.into_function().into_pull_type().into_dyn_pin(),
                r2: pins.gpio3.into_function().into_pull_type().into_dyn_pin(),
                g2: pins.gpio4.into_function().into_pull_type().into_dyn_pin(),
                b2: pins.gpio5.into_function().into_pull_type().into_dyn_pin(),
                addr: [
                    pins.gpio6.into_function().into_pull_type().into_dyn_pin(),
                    pins.gpio7.into_function().into_pull_type().into_dyn_pin(),
                    pins.gpio8.into_function().into_pull_type().into_dyn_pin(),
                    pins.gpio9.into_function().into_pull_type().into_dyn_pin(),
                    pins.gpio10.into_function().into_pull_type().into_dyn_pin(),
                ],
                clk: pins.gpio11.into_function().into_pull_type().into_dyn_pin(),
                lat: pins.gpio12.into_function().into_pull_type().into_dyn_pin(),
                oe: pins.gpio13.into_function().into_pull_type().into_dyn_pin(),
            },
            &mut pio,
            (sm0, sm1, sm2),
            (dma.ch0, dma.ch1, dma.ch2, dma.ch3),
            false,
            LUT.as_ref().unwrap() 
        )
    };

    let hub75_ref = HUB75.init(Mutex::new(display));
    
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

/*
PIN CONFIGURATION

RGB Data (6 consecutive):
GPIO 0 → R1
GPIO 1 → G1
GPIO 2 → B1
GPIO 3 → R2
GPIO 4 → G2
GPIO 5 → B2


Address (5 consecutive):
GPIO 6 → A
GPIO 7 → B
GPIO 8 → C
GPIO 9 → D
GPIO 10 → E

Control:
GPIO 11 → CLK
GPIO 12 → LAT/STB
GPIO 13 → OE
*/