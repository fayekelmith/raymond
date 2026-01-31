use embedded_graphics::pixelcolor::Rgb888;

/// Predefined neon color palette for LED matrix display
pub const NEON_PALETTE: [Rgb888; 8] = [
    Rgb888::new(0, 255, 255),       // Cyan
    Rgb888::new(255, 0, 255),       // Magenta
    Rgb888::new(255, 255, 0),       // Yellow
    Rgb888::new(0, 255, 0),         // Lime
    Rgb888::new(255, 20, 147),      // Hot Pink
    Rgb888::new(0, 191, 255),       // Electric Blue
    Rgb888::new(255, 165, 0),       // Orange
    Rgb888::new(128, 0, 255),       // Purple
];

/// Linearly interpolate between two RGB colors
/// 
/// # Arguments
/// * `from` - Starting color
/// * `to` - Ending color
/// * `t` - Interpolation factor (0.0 = from, 1.0 = to)
/// 
/// # Returns
/// Interpolated RGB color
pub fn interpolate_rgb(from: Rgb888, to: Rgb888, t: f32) -> Rgb888 {
    let t_clamped = t.clamp(0.0, 1.0);
    
    let r = from.r() as f32 + ((to.r() as i16 - from.r() as i16) as f32 * t_clamped);
    let g = from.g() as f32 + ((to.g() as i16 - from.g() as i16) as f32 * t_clamped);
    let b = from.b() as f32 + ((to.b() as i16 - from.b() as i16) as f32 * t_clamped);
    
    Rgb888::new(
        r.round() as u8,
        g.round() as u8,
        b.round() as u8,
    )
}

/// Manages color palette cycling state
pub struct ColorState {
    pub current_idx: u8,
    pub next_idx: u8,
    pub progress: f32,
}

impl ColorState {
    /// Create a new ColorState starting at the beginning of the palette
    pub fn new() -> Self {
        Self {
            current_idx: 0,
            next_idx: 1,
            progress: 0.0,
        }
    }
    
    /// Advance to the next color pair in the palette and reset progress
    pub fn advance(&mut self) {
        self.current_idx = self.next_idx;
        self.next_idx = (self.next_idx + 1) % NEON_PALETTE.len() as u8;
        self.progress = 0.0;
    }
    
    /// Get the current interpolated color based on progress
    pub fn get_color(&self) -> Rgb888 {
        let from = NEON_PALETTE[self.current_idx as usize];
        let to = NEON_PALETTE[self.next_idx as usize];
        interpolate_rgb(from, to, self.progress)
    }
}

impl Default for ColorState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interpolate_rgb_at_start() {
        let from = Rgb888::new(0, 0, 0);
        let to = Rgb888::new(255, 255, 255);
        let result = interpolate_rgb(from, to, 0.0);
        
        assert_eq!(result.r(), 0);
        assert_eq!(result.g(), 0);
        assert_eq!(result.b(), 0);
    }

    #[test]
    fn test_interpolate_rgb_at_end() {
        let from = Rgb888::new(0, 0, 0);
        let to = Rgb888::new(255, 255, 255);
        let result = interpolate_rgb(from, to, 1.0);
        
        assert_eq!(result.r(), 255);
        assert_eq!(result.g(), 255);
        assert_eq!(result.b(), 255);
    }

    #[test]
    fn test_interpolate_rgb_at_midpoint() {
        let from = Rgb888::new(0, 0, 0);
        let to = Rgb888::new(100, 200, 50);
        let result = interpolate_rgb(from, to, 0.5);
        
        assert_eq!(result.r(), 50);
        assert_eq!(result.g(), 100);
        assert_eq!(result.b(), 25);
    }

    #[test]
    fn test_interpolate_rgb_with_actual_palette_colors() {
        // Test cyan to magenta at midpoint
        let cyan = Rgb888::new(0, 255, 255);
        let magenta = Rgb888::new(255, 0, 255);
        let result = interpolate_rgb(cyan, magenta, 0.5);
        
        assert_eq!(result.r(), 128);  // (0 + 255) / 2
        assert_eq!(result.g(), 128);  // (255 + 0) / 2
        assert_eq!(result.b(), 255);  // (255 + 255) / 2
    }

    #[test]
    fn test_color_state_initialization() {
        let state = ColorState::new();
        
        assert_eq!(state.current_idx, 0);
        assert_eq!(state.next_idx, 1);
        assert_eq!(state.progress, 0.0);
    }

    #[test]
    fn test_color_state_advance() {
        let mut state = ColorState::new();
        state.progress = 0.75;
        
        state.advance();
        
        assert_eq!(state.current_idx, 1);
        assert_eq!(state.next_idx, 2);
        assert_eq!(state.progress, 0.0);
    }

    #[test]
    fn test_color_state_cycles_correctly() {
        let mut state = ColorState::new();
        
        // Advance through all colors
        for i in 0..8 {
            assert_eq!(state.current_idx, i);
            assert_eq!(state.next_idx, (i + 1) % 8);
            state.advance();
        }
        
        // Should wrap back to start
        assert_eq!(state.current_idx, 0);
        assert_eq!(state.next_idx, 1);
    }

    #[test]
    fn test_get_color_at_start_of_transition() {
        let mut state = ColorState::new();
        state.progress = 0.0;
        
        let color = state.get_color();
        let expected = NEON_PALETTE[0];  // Cyan
        
        assert_eq!(color.r(), expected.r());
        assert_eq!(color.g(), expected.g());
        assert_eq!(color.b(), expected.b());
    }

    #[test]
    fn test_get_color_at_end_of_transition() {
        let mut state = ColorState::new();
        state.progress = 1.0;
        
        let color = state.get_color();
        let expected = NEON_PALETTE[1];  // Magenta
        
        assert_eq!(color.r(), expected.r());
        assert_eq!(color.g(), expected.g());
        assert_eq!(color.b(), expected.b());
    }

    #[test]
    fn test_get_color_at_midpoint() {
        let mut state = ColorState::new();
        state.progress = 0.5;
        
        let color = state.get_color();
        
        // Should be halfway between cyan (0,255,255) and magenta (255,0,255)
        assert_eq!(color.r(), 128);
        assert_eq!(color.g(), 128);
        assert_eq!(color.b(), 255);
    }

    #[test]
    fn test_neon_palette_has_correct_colors() {
        assert_eq!(NEON_PALETTE[0], Rgb888::new(0, 255, 255));     // Cyan
        assert_eq!(NEON_PALETTE[1], Rgb888::new(255, 0, 255));     // Magenta
        assert_eq!(NEON_PALETTE[2], Rgb888::new(255, 255, 0));     // Yellow
        assert_eq!(NEON_PALETTE[3], Rgb888::new(0, 255, 0));       // Lime
        assert_eq!(NEON_PALETTE[4], Rgb888::new(255, 20, 147));    // Hot Pink
        assert_eq!(NEON_PALETTE[5], Rgb888::new(0, 191, 255));     // Electric Blue
        assert_eq!(NEON_PALETTE[6], Rgb888::new(255, 165, 0));     // Orange
        assert_eq!(NEON_PALETTE[7], Rgb888::new(128, 0, 255));     // Purple
    }
}
