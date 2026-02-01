//! Animation system with easing functions for smooth cell transitions
//!
//! Provides smooth fade-in/fade-out transitions for cell birth/death
//! and brightness modulation over 300ms using cubic easing.

use core::default::Default;
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::RgbColor;
use heapless::FnvIndexSet;

/// Cubic easing function (smoothstep) for smooth transitions
///
/// Returns smoothed value in range [0.0, 1.0] for input t in [0.0, 1.0]
/// Uses formula: t * t * (3.0 - 2.0 * t)
pub fn ease_in_out(t: f32) -> f32 {
    t * t * (3.0 - 2.0 * t)
}

/// Apply brightness factor to an RGB color
///
/// Scales each RGB channel by brightness factor (0.0 to 1.0)
/// and clamps result to 0-255 range
pub fn apply_brightness(color: Rgb888, brightness: f32) -> Rgb888 {
    let r = (color.r() as f32 * brightness).clamp(0.0, 255.0) as u8;
    let g = (color.g() as f32 * brightness).clamp(0.0, 255.0) as u8;
    let b = (color.b() as f32 * brightness).clamp(0.0, 255.0) as u8;

    Rgb888::new(r, g, b)
}

/// Tracks cell animations for smooth birth/death transitions
///
/// Uses heapless collections to avoid heap allocations in embedded environment.
/// Total memory: ~2KB (two 256-element sets + progress float)
pub struct CellAnimator {
    /// Cells being born (fading in)
    pub(crate) birth_cells: FnvIndexSet<(u8, u8), 256>,
    /// Cells dying (fading out)
    pub(crate) death_cells: FnvIndexSet<(u8, u8), 256>,
    /// Animation progress from 0.0 to 1.0 (over 300ms)
    pub(crate) progress: f32,
}

impl CellAnimator {
    /// Create a new CellAnimator with empty transition sets
    pub fn new() -> Self {
        Self {
            birth_cells: FnvIndexSet::new(),
            death_cells: FnvIndexSet::new(),
            progress: 0.0,
        }
    }

    /// Start a new transition with the given cell changes
    ///
    /// # Arguments
    /// * `changes` - Slice of (x, y, is_birth) tuples indicating cell state changes
    ///
    /// Clears previous transition state and populates birth/death sets.
    /// Sets progress to 0.0 to begin new animation cycle.
    pub fn start_transition(&mut self, changes: &[(u8, u8, bool)]) {
        self.birth_cells.clear();
        self.death_cells.clear();
        self.progress = 0.0;

        for &(x, y, is_birth) in changes {
            if is_birth {
                let _ = self.birth_cells.insert((x, y));
            } else {
                let _ = self.death_cells.insert((x, y));
            }
        }
    }

    /// Update animation progress based on elapsed time
    ///
    /// # Arguments
    /// * `delta_ms` - Milliseconds elapsed since last update
    ///
    /// Increments progress from 0.0 to 1.0 over 300ms total duration.
    /// Progress is clamped to [0.0, 1.0] range.
    pub fn update(&mut self, delta_ms: u32) {
        const TRANSITION_DURATION_MS: f32 = 300.0;

        self.progress += delta_ms as f32 / TRANSITION_DURATION_MS;
        self.progress = self.progress.clamp(0.0, 1.0);
    }

    /// Get brightness factor for a cell at current animation progress
    ///
    /// # Arguments
    /// * `x` - Cell x coordinate
    /// * `y` - Cell y coordinate
    /// * `is_alive` - Whether cell is currently alive
    ///
    /// # Returns
    /// Brightness factor in range [0.0, 1.0]:
    /// - Birth cells: 0.0 → 1.0 (fade in)
    /// - Death cells: 1.0 → 0.0 (fade out)
    /// - Persistent alive cells: 1.0 (full brightness)
    /// - Persistent dead cells: 0.0 (black)
    ///
    /// Applies easing function to progress for smooth transitions.
    pub fn get_cell_brightness(&self, x: u8, y: u8, is_alive: bool) -> f32 {
        let cell_pos = (x, y);
        let eased_progress = ease_in_out(self.progress);

        if self.birth_cells.contains(&cell_pos) {
            // Birth: fade from 0.0 to 1.0
            eased_progress
        } else if self.death_cells.contains(&cell_pos) {
            // Death: fade from 1.0 to 0.0
            1.0 - eased_progress
        } else if is_alive {
            // Persistent alive: full brightness
            1.0
        } else {
            // Persistent dead: no brightness
            0.0
        }
    }
}

impl Default for CellAnimator {
    fn default() -> Self {
        Self::new()
    }
}
