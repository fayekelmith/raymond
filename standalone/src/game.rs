//! Conway's Game of Life implementation using bit-packed arrays
//!
//! Memory layout: 64x64 grid = 4096 cells = 512 bytes per grid
//! Three grids (current, next, previous) = 1536 bytes total

/// Game of Life board with bit-packed state storage
///
/// Each grid is 512 bytes representing a 64x64 cell grid (1 bit per cell)
/// Memory layout: exactly 1536 bytes
pub struct GameBoard {
    /// Current generation state
    pub current: [u8; 512],
    /// Next generation buffer (double-buffering)
    pub next: [u8; 512],
    /// Previous generation for transition detection
    pub previous: [u8; 512],
}

impl GameBoard {
    /// Create a new empty game board
    pub fn new() -> Self {
        Self {
            current: [0u8; 512],
            next: [0u8; 512],
            previous: [0u8; 512],
        }
    }

    /// Get the state of a cell at position (x, y)
    ///
    /// Uses bit operations: reads from current grid
    /// Formula: grid[(y * 8) + (x / 8)] >> (x % 8) & 1
    #[inline]
    pub fn get_cell(&self, x: u8, y: u8) -> bool {
        let byte_idx = (y as usize * 8) + (x as usize / 8);
        let bit_idx = x % 8;
        (self.current[byte_idx] >> bit_idx) & 1 != 0
    }

    /// Set the state of a cell in the current grid
    ///
    /// Uses bit masking to set or clear individual bits
    pub fn set_cell_current(&mut self, x: u8, y: u8, alive: bool) {
        let byte_idx = (y as usize * 8) + (x as usize / 8);
        let bit_idx = x % 8;

        if alive {
            // Set bit: OR with mask
            self.current[byte_idx] |= 1 << bit_idx;
        } else {
            // Clear bit: AND with inverted mask
            self.current[byte_idx] &= !(1 << bit_idx);
        }
    }

    /// Set the state of a cell in the next grid (internal helper)
    #[inline]
    fn set_cell_next(&mut self, x: u8, y: u8, alive: bool) {
        let byte_idx = (y as usize * 8) + (x as usize / 8);
        let bit_idx = x % 8;

        if alive {
            self.next[byte_idx] |= 1 << bit_idx;
        } else {
            self.next[byte_idx] &= !(1 << bit_idx);
        }
    }

    /// Count living neighbors for a cell with wrapping boundaries
    ///
    /// Checks all 8 adjacent cells, wrapping at edges for toroidal topology
    pub fn count_neighbors(&self, x: u8, y: u8) -> u8 {
        let mut count = 0u8;

        // Neighbor offsets (dx, dy)
        let offsets: [(i8, i8); 8] = [
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];

        for (dx, dy) in offsets.iter() {
            // Wrapping arithmetic for toroidal boundaries (64x64 grid)
            let nx = ((x as i16 + *dx as i16 + 64) % 64) as u8;
            let ny = ((y as i16 + *dy as i16 + 64) % 64) as u8;

            if self.get_cell(nx, ny) {
                count += 1;
            }
        }

        count
    }

    /// Update to next generation using Conway's rules
    ///
    /// Rules:
    /// - Alive cell with 2-3 neighbors: survives
    /// - Dead cell with exactly 3 neighbors: becomes alive
    /// - All other cells: die
    #[cfg(not(test))]
    pub fn update_generation(&mut self) -> heapless::Vec<(u8, u8, bool), 256> {
        let mut changes: heapless::Vec<(u8, u8, bool), 256> = heapless::Vec::new();

        // Copy current to previous for transition tracking
        self.previous.copy_from_slice(&self.current);

        // Apply Conway's rules to compute next generation
        for y in 0..64u8 {
            for x in 0..64u8 {
                let alive = self.get_cell(x, y);
                let neighbors = self.count_neighbors(x, y);

                let next_state = match (alive, neighbors) {
                    (true, 2) | (true, 3) => true, // Survival
                    (false, 3) => true,            // Birth
                    _ => false,                    // Death
                };

                self.set_cell_next(x, y, next_state);

                // Track changes for animation
                if alive != next_state {
                    // Only track up to 256 changes (heapless limit)
                    if changes.push((x, y, next_state)).is_err() {
                        // Silently ignore additional changes if buffer full
                        break;
                    }
                }
            }
        }

        // Swap current and next buffers
        core::mem::swap(&mut self.current, &mut self.next);

        changes
    }

    /// Update to next generation (test version without change tracking)
    #[cfg(test)]
    pub fn update_generation(&mut self) {
        // Copy current to previous for transition tracking
        self.previous.copy_from_slice(&self.current);

        // Apply Conway's rules to compute next generation
        for y in 0..64u8 {
            for x in 0..64u8 {
                let alive = self.get_cell(x, y);
                let neighbors = self.count_neighbors(x, y);

                let next_state = match (alive, neighbors) {
                    (true, 2) | (true, 3) => true, // Survival
                    (false, 3) => true,            // Birth
                    _ => false,                    // Death
                };

                self.set_cell_next(x, y, next_state);
            }
        }

        // Swap current and next buffers
        core::mem::swap(&mut self.current, &mut self.next);
    }

    /// Initialize the board with random cell states
    ///
    /// Sets approximately 30% of cells to alive using fastrand
    #[cfg(not(test))]
    pub fn random_init(&mut self, seed: u64) {
        let mut rng = fastrand::Rng::with_seed(seed);
        for y in 0..64u8 {
            for x in 0..64u8 {
                // 30% chance of being alive
                let alive = rng.u8(0..100) < 30;
                self.set_cell_current(x, y, alive);
            }
        }
    }
}
