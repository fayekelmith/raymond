//! Animation system with easing functions for smooth cell transitions
//!
//! Provides smooth fade-in/fade-out transitions for cell birth/death
//! and brightness modulation over 300ms using cubic easing.

use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::RgbColor;
use heapless::FnvIndexSet;
use core::default::Default;

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
    birth_cells: FnvIndexSet<(u8, u8), 256>,
    /// Cells dying (fading out)
    death_cells: FnvIndexSet<(u8, u8), 256>,
    /// Animation progress from 0.0 to 1.0 (over 300ms)
    progress: f32,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ease_in_out_boundaries() {
        assert_eq!(ease_in_out(0.0), 0.0, "ease_in_out(0.0) should be 0.0");
        assert_eq!(ease_in_out(1.0), 1.0, "ease_in_out(1.0) should be 1.0");
    }

    #[test]
    fn test_ease_in_out_midpoint() {
        let mid = ease_in_out(0.5);
        assert!((mid - 0.5).abs() < 0.01, "ease_in_out(0.5) should be approximately 0.5, got {}", mid);
    }

    #[test]
    fn test_ease_in_out_smooth_curve() {
        // Verify it produces a smooth curve (not linear)
        let quarter = ease_in_out(0.25);
        let three_quarter = ease_in_out(0.75);
        
        // For smoothstep, these should not be exactly 0.25 and 0.75
        assert!(quarter < 0.25, "Easing should slow at start");
        assert!(three_quarter > 0.75, "Easing should slow at end");
    }

    #[test]
    fn test_apply_brightness_full() {
        let color = Rgb888::new(200, 150, 100);
        let result = apply_brightness(color, 1.0);
        assert_eq!(result.r(), 200);
        assert_eq!(result.g(), 150);
        assert_eq!(result.b(), 100);
    }

    #[test]
    fn test_apply_brightness_half() {
        let color = Rgb888::new(200, 150, 100);
        let result = apply_brightness(color, 0.5);
        assert_eq!(result.r(), 100);
        assert_eq!(result.g(), 75);
        assert_eq!(result.b(), 50);
    }

    #[test]
    fn test_apply_brightness_zero() {
        let color = Rgb888::new(200, 150, 100);
        let result = apply_brightness(color, 0.0);
        assert_eq!(result.r(), 0);
        assert_eq!(result.g(), 0);
        assert_eq!(result.b(), 0);
    }

    #[test]
    fn test_apply_brightness_clamping() {
        let color = Rgb888::new(200, 150, 100);
        let result = apply_brightness(color, 1.5);
        // Should clamp to 255 max
        assert_eq!(result.r(), 255);
        assert_eq!(result.g(), 225);
        assert_eq!(result.b(), 150);
    }

    #[test]
    fn test_cell_animator_new() {
        let animator = CellAnimator::new();
        assert_eq!(animator.progress, 0.0);
        assert!(animator.birth_cells.is_empty());
        assert!(animator.death_cells.is_empty());
    }

    #[test]
    fn test_cell_animator_start_transition() {
        let mut animator = CellAnimator::new();
        let changes = [(10, 20, true), (15, 25, false), (5, 5, true)];
        
        animator.start_transition(&changes);
        
        assert_eq!(animator.progress, 0.0);
        assert!(animator.birth_cells.contains(&(10, 20)));
        assert!(animator.birth_cells.contains(&(5, 5)));
        assert!(animator.death_cells.contains(&(15, 25)));
        assert_eq!(animator.birth_cells.len(), 2);
        assert_eq!(animator.death_cells.len(), 1);
    }

    #[test]
    fn test_cell_animator_update_progress() {
        let mut animator = CellAnimator::new();
        
        // 300ms total duration
        animator.update(100);
        assert!((animator.progress - 0.333).abs() < 0.01);
        
        animator.update(100);
        assert!((animator.progress - 0.666).abs() < 0.01);
        
        animator.update(100);
        assert_eq!(animator.progress, 1.0);
        
        // Should clamp at 1.0
        animator.update(100);
        assert_eq!(animator.progress, 1.0);
    }

    #[test]
    fn test_cell_animator_brightness_birth() {
        let mut animator = CellAnimator::new();
        animator.start_transition(&[(10, 20, true)]);
        
        // At progress 0.0, birth cell should have 0.0 brightness
        let brightness_start = animator.get_cell_brightness(10, 20, true);
        assert_eq!(brightness_start, 0.0);
        
        // At progress 1.0, birth cell should have 1.0 brightness
        animator.progress = 1.0;
        let brightness_end = animator.get_cell_brightness(10, 20, true);
        assert_eq!(brightness_end, 1.0);
        
        // At progress 0.5, should be approximately 0.5 (with easing)
        animator.progress = 0.5;
        let brightness_mid = animator.get_cell_brightness(10, 20, true);
        assert!((brightness_mid - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_cell_animator_brightness_death() {
        let mut animator = CellAnimator::new();
        animator.start_transition(&[(15, 25, false)]);
        
        // At progress 0.0, death cell should have 1.0 brightness
        let brightness_start = animator.get_cell_brightness(15, 25, false);
        assert_eq!(brightness_start, 1.0);
        
        // At progress 1.0, death cell should have 0.0 brightness
        animator.progress = 1.0;
        let brightness_end = animator.get_cell_brightness(15, 25, false);
        assert_eq!(brightness_end, 0.0);
        
        // At progress 0.5, should be approximately 0.5 (with easing)
        animator.progress = 0.5;
        let brightness_mid = animator.get_cell_brightness(15, 25, false);
        assert!((brightness_mid - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_cell_animator_brightness_persistent_alive() {
        let animator = CellAnimator::new();
        
        // Cell not in birth or death sets, but is alive
        let brightness = animator.get_cell_brightness(30, 40, true);
        assert_eq!(brightness, 1.0);
    }

    #[test]
    fn test_cell_animator_brightness_persistent_dead() {
        let animator = CellAnimator::new();
        
        // Cell not in birth or death sets, and is dead
        let brightness = animator.get_cell_brightness(30, 40, false);
        assert_eq!(brightness, 0.0);
    }
}
