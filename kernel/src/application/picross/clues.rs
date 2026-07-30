/*
 * Calculation and storage of Picross row and column clues.
 */

use super::level::PicrossLevel;
use super::BOARD_SIZE;

/// Maximum number of clue groups possible in a ten-cell line.
pub const MAX_CLUES: usize = 5;

/// The clues belonging to one row or column.
///
/// Only the first `length` entries contain valid values.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LineClue {
    values: [u8; MAX_CLUES],
    length: usize,
}

impl LineClue {
    /// Create an empty clue.
    pub const fn empty() -> Self {
        Self {
            values: [0; MAX_CLUES],
            length: 0,
        }
    }

    /// Number of valid clue values.
    pub fn len(&self) -> usize {
        self.length
    }

    /// Get one clue value.
    pub fn value(&self, index: usize) -> u8 {
        self.values[index]
    }

    /// Add another group length.
    fn push(&mut self, value: u8) {
        if self.length >= MAX_CLUES {
            return;
        }

        self.values[self.length] = value;
        self.length += 1;
    }
}

/// All clues belonging to a level.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Clues {
    pub rows: [LineClue; BOARD_SIZE],
    pub columns: [LineClue; BOARD_SIZE],
}

impl Clues {
    /// Create an empty clue set.
    pub const fn empty() -> Self {
        Self {
            rows: [LineClue::empty(); BOARD_SIZE],
            columns: [LineClue::empty(); BOARD_SIZE],
        }
    }

    /// Calculate every row and column clue for a level.
    pub fn calculate(level: &PicrossLevel) -> Self {
        let mut clues = Self::empty();

        for y in 0..BOARD_SIZE {
            clues.rows[y] = calculate_line(|x| level.is_filled(x, y));
        }

        for x in 0..BOARD_SIZE {
            clues.columns[x] = calculate_line(|y| level.is_filled(x, y));
        }

        clues
    }
}

/// Calculate the clue groups for one ten-cell line.
fn calculate_line<F>(mut is_filled: F) -> LineClue
where
    F: FnMut(usize) -> bool,
{
    let mut clue = LineClue::empty();
    let mut current_group = 0;

    for index in 0..BOARD_SIZE {
        if is_filled(index) {
            current_group += 1;
        } else if current_group > 0 {
            clue.push(current_group);
            current_group = 0;
        }
    }

    // The line may end with a filled group.
    if current_group > 0 {
        clue.push(current_group);
    }

    // Display an empty line as zero.
    if clue.len() == 0 {
        clue.push(0);
    }

    clue
}