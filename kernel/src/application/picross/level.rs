/*
 * Picross level definitions.
 */

use super::BOARD_SIZE;
pub const LEVEL_COUNT: usize = 3;

/// A single 10x10 Picross level.
///
/// A '#' represents a cell that must be carved.
/// A '.' represents a cell that must remain empty.
pub struct PicrossLevel {
    name: &'static str,
    solution: [&'static str; BOARD_SIZE],
    time_limit_seconds: usize,
}

impl PicrossLevel {
    pub const fn new(name: &'static str, solution: [&'static str; BOARD_SIZE], time_limit_seconds: usize) -> Self {
        Self {
            name,
            solution,
            time_limit_seconds,
        }
    }

    /// Get the display name of the level.
    pub fn name(&self) -> &'static str {
        self.name
    }

    /// Get the level's time limit in seconds.
    pub fn time_limit_seconds(&self) -> usize {
        self.time_limit_seconds
    }

    /// Check whether a specific solution cell must be carved.
    pub fn is_filled(&self, x: usize, y: usize) -> bool {
        self.solution[y].as_bytes()[x] == b'#'
    }
}

/// All currently available Picross levels.
pub const LEVELS: [PicrossLevel; LEVEL_COUNT] = [
    PicrossLevel::new(
        "1A",
        [
            "..##..##..",
            ".########.",
            "##########",
            "##########",
            ".########.",
            "..######..",
            "...####...",
            "....##....",
            "..........",
            "..........",
        ],
        600,
    ),
    PicrossLevel::new(
        "2A",
        [
            "...####...",
            "..######..",
            ".########.",
            "##########",
            "##.####.##",
            "...####...",
            "...####...",
            "..######..",
            "..##..##..",
            "..........",
        ],
        600,
    ),
    PicrossLevel::new(
        "3A",
        [
            "..#....#..",
            "...#..#...",
            "..######..",
            ".##.##.##.",
            "##########",
            "#.######.#",
            "#.#....#.#",
            "...#..#...",
            "..#....#..",
            "..........",
        ],
        600,
    ),
];