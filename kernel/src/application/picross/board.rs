/*
 * Data structures for the visible Picross board.
 */

use super::BOARD_SIZE;

/// The state of one cell as seen by the player.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellState {
    /// The player has not interacted with this cell.
    Uncarved,

    /// The player believes this cell should be carved.
    Marked,

    /// The cell has been carved successfully.
    ///
    /// A carved cell cannot be changed again.
    Carved,
}

/// The player-visible 10x10 board.
pub struct Board {
    cells: [[CellState; BOARD_SIZE]; BOARD_SIZE],
}

impl Board {
    /// Create a completely empty board.
    pub const fn new() -> Self {
        Self {
            cells: [[CellState::Uncarved; BOARD_SIZE]; BOARD_SIZE],
        }
    }

    /// Get the current state of a cell.
    pub fn cell(&self, x: usize, y: usize) -> CellState {
        self.cells[y][x]
    }

    /// Mark or unmark a cell.
    ///
    /// Carved cells cannot be changed.
    pub fn toggle_mark(&mut self, x: usize, y: usize) {
        match self.cells[y][x] {
            CellState::Uncarved => {
                self.cells[y][x] = CellState::Marked;
            }
            CellState::Marked => {
                self.cells[y][x] = CellState::Uncarved;
            }
            CellState::Carved => {
                // Carved cells are permanent.
            }
        }
    }

    /// Permanently carve a cell.
    ///
    /// Whether the move is correct will later be checked by the game logic.
    pub fn carve(&mut self, x: usize, y: usize) {
        self.cells[y][x] = CellState::Carved;
    }

    /// Reset every cell to its initial state.
    pub fn clear(&mut self) {
        self.cells = [[CellState::Uncarved; BOARD_SIZE]; BOARD_SIZE];
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}