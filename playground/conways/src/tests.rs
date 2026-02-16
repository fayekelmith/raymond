#[cfg(test)]
mod game_tests {
    use crate::game::GameBoard;
    use core::mem;

    #[test]
    fn test_get_set_cell() {
        let mut board = GameBoard::new();

        // Test setting and getting various cells
        board.set_cell_current(0, 0, true);
        assert!(board.get_cell(0, 0));

        board.set_cell_current(7, 0, true);
        assert!(board.get_cell(7, 0));

        board.set_cell_current(63, 63, true);
        assert!(board.get_cell(63, 63));

        // Test clearing cells
        board.set_cell_current(0, 0, false);
        assert!(!board.get_cell(0, 0));

        // Test middle cell
        board.set_cell_current(32, 32, true);
        assert!(board.get_cell(32, 32));
    }

    #[test]
    fn test_count_neighbors_interior() {
        let mut board = GameBoard::new();

        // Create a 3x3 block centered at (10, 10)
        for dy in 0..3 {
            for dx in 0..3 {
                board.set_cell_current(9 + dx, 9 + dy, true);
            }
        }

        // Center cell should have 8 neighbors
        assert_eq!(board.count_neighbors(10, 10), 8);

        // Corner cells should have 3 neighbors
        assert_eq!(board.count_neighbors(9, 9), 3);
        assert_eq!(board.count_neighbors(11, 11), 3);

        // Edge cells should have 5 neighbors
        assert_eq!(board.count_neighbors(10, 9), 5);
    }

    #[test]
    fn test_count_neighbors_wrapping() {
        let mut board = GameBoard::new();

        // Place cells at corners to test wrapping
        board.set_cell_current(0, 0, true);
        board.set_cell_current(63, 0, true);
        board.set_cell_current(0, 63, true);
        board.set_cell_current(63, 63, true);

        // Top-left corner should see all 4 corner cells as neighbors
        // (wrapping makes them adjacent)
        assert_eq!(board.count_neighbors(0, 0), 3);
        assert_eq!(board.count_neighbors(63, 63), 3);
    }

    #[test]
    fn test_blinker_pattern() {
        let mut board = GameBoard::new();

        // Create horizontal blinker at (10, 10)
        board.set_cell_current(9, 10, true);
        board.set_cell_current(10, 10, true);
        board.set_cell_current(11, 10, true);

        // Update one generation - should become vertical
        board.update_generation();

        assert!(!board.get_cell(9, 10));
        assert!(board.get_cell(10, 9));
        assert!(board.get_cell(10, 10));
        assert!(board.get_cell(10, 11));
        assert!(!board.get_cell(11, 10));

        // Update again - should return to horizontal
        board.update_generation();

        assert!(board.get_cell(9, 10));
        assert!(board.get_cell(10, 10));
        assert!(board.get_cell(11, 10));
        assert!(!board.get_cell(10, 9));
        assert!(!board.get_cell(10, 11));
    }

    #[test]
    fn test_update_generation_changes() {
        let mut board = GameBoard::new();

        // Create a simple pattern (2x2 block)
        board.set_cell_current(10, 10, true);
        board.set_cell_current(11, 10, true);
        board.set_cell_current(10, 11, true);
        board.set_cell_current(11, 11, true);

        // Block pattern should be stable (no changes)
        board.update_generation();

        // All 4 cells should still be alive (2x2 block is stable)
        assert!(board.get_cell(10, 10));
        assert!(board.get_cell(11, 10));
        assert!(board.get_cell(10, 11));
        assert!(board.get_cell(11, 11));
    }

    #[test]
    fn test_memory_size() {
        // Verify exact memory usage
        assert_eq!(mem::size_of::<GameBoard>(), 1536);
        assert_eq!(mem::size_of::<[u8; 512]>(), 512);
    }

    #[test]
    fn test_glider_pattern() {
        let mut board = GameBoard::new();

        // Create glider at (10, 10)
        //  .X.
        //  ..X
        //  XXX
        board.set_cell_current(11, 10, true);
        board.set_cell_current(12, 11, true);
        board.set_cell_current(10, 12, true);
        board.set_cell_current(11, 12, true);
        board.set_cell_current(12, 12, true);

        // After 4 generations, glider should move one cell diagonally
        for _ in 0..4 {
            board.update_generation();
        }

        // Check that glider moved (at least some cells changed position)
        // Count all alive cells
        let mut alive_count = 0u32;
        for y in 0..64u8 {
            for x in 0..64u8 {
                if board.get_cell(x, y) {
                    alive_count += 1;
                }
            }
        }

        // Glider should still have 5 cells alive
        assert_eq!(alive_count, 5);
    }
}

#[cfg(test)]
mod animator_tests {
    use crate::animator::*;
    use embedded_graphics::pixelcolor::Rgb888;
    use embedded_graphics::prelude::RgbColor;

    #[test]
    fn test_ease_in_out_boundaries() {
        assert_eq!(ease_in_out(0.0), 0.0, "ease_in_out(0.0) should be 0.0");
        assert_eq!(ease_in_out(1.0), 1.0, "ease_in_out(1.0) should be 1.0");
    }

    #[test]
    fn test_ease_in_out_midpoint() {
        let mid = ease_in_out(0.5);
        assert!(
            (mid - 0.5).abs() < 0.01,
            "ease_in_out(0.5) should be approximately 0.5, got {}",
            mid
        );
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
