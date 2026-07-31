/*
 * Main game state for Picross.
 */
use crate::device::pit::wait;
use super::board::{Board, CellState};
use super::level::{PicrossLevel, LEVELS};
use super::clues::Clues;
use super::BOARD_SIZE;
use super::timer::{GameTimer, INCORRECT_CARVE_PENALTY_SECONDS};
use super::highscore::highscores;

/// The screen Picross is currently displaying.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PicrossScreen {
    LevelSelection,
    Playing,
    Completed,
    Failed,
}

/// The result of attempting to carve the selected cell.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CarveResult {
    /// The selected cell was correct and has been carved.
    Correct,

    /// The selected cell should not be carved.
    Incorrect,

    /// The selected cell was already carved.
    AlreadyCarved,

    /// Carving was attempted outside the playing screen.
    NotPlaying,
}

/// Position of the currently selected board cell.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cursor {
    pub x: usize,
    pub y: usize,
}

impl Cursor {
    /// Create a cursor at the top-left cell.
    pub const fn new() -> Self {
        Self { x: 0, y: 0 }
    }

    pub fn move_up(&mut self) {
        if self.y > 0 {
            self.y -= 1;
        }
    }

    pub fn move_down(&mut self) {
        if self.y + 1 < BOARD_SIZE {
            self.y += 1;
        }
    }

    pub fn move_left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }

    pub fn move_right(&mut self) {
        if self.x + 1 < BOARD_SIZE {
            self.x += 1;
        }
    }
}

impl Default for Cursor {
    fn default() -> Self {
        Self::new()
    }
}

/// The complete changing state of the Picross application.
pub struct PicrossGame {
    pub screen: PicrossScreen,
    pub board: Board,
    pub cursor: Cursor,
    pub selected_level: usize,
    pub clues: Clues,
    pub timer: GameTimer,
    /// Final completion time of the most recently completed level.
    pub completion_time_seconds: Option<usize>,

    /// Whether the most recent completion set a new record.
    pub new_highscore: bool,
}

impl PicrossGame {
    /// Create a new Picross application.
    pub const fn new() -> Self {
        Self {
            screen: PicrossScreen::LevelSelection,
            board: Board::new(),
            cursor: Cursor::new(),
            selected_level: 0,
            clues: Clues::empty(),
            timer: GameTimer::new(),
            completion_time_seconds: None,
            new_highscore: false,
        }
    }

    /// Get the currently selected level.
    pub fn current_level(&self) -> &'static PicrossLevel {
        &LEVELS[self.selected_level]
    }

    /// Start a level and reset the player's progress.
    pub fn start_level(&mut self, level: usize, now_ms: usize) {
        if level >= LEVELS.len() {
            return;
        }

        self.selected_level = level;
        self.board.clear();
        self.cursor = Cursor::new();
        self.clues = Clues::calculate(&LEVELS[level]);

        self.completion_time_seconds = None;
        self.new_highscore = false;

        self.timer.start(now_ms, LEVELS[level].time_limit_seconds());

        self.screen = PicrossScreen::Playing;
    }

    pub fn move_cursor_up(&mut self) {
        if self.screen == PicrossScreen::Playing {
            self.cursor.move_up();
        }
    }

    pub fn move_cursor_down(&mut self) {
        if self.screen == PicrossScreen::Playing {
            self.cursor.move_down();
        }
    }

    pub fn move_cursor_left(&mut self) {
        if self.screen == PicrossScreen::Playing {
            self.cursor.move_left();
        }
    }

    pub fn move_cursor_right(&mut self) {
        if self.screen == PicrossScreen::Playing {
            self.cursor.move_right();
        }
    }

    /// Mark or unmark the currently selected cell.
    pub fn toggle_selected_mark(&mut self) {
        if self.screen != PicrossScreen::Playing {
            return;
        }

        self.board.toggle_mark(self.cursor.x, self.cursor.y);
    }

    /// Attempt to carve the currently selected cell.
    pub fn carve_selected(&mut self, now_ms: usize) -> CarveResult {
        if self.screen != PicrossScreen::Playing {
            return CarveResult::NotPlaying;
        }

        let x = self.cursor.x;
        let y = self.cursor.y;

        if self.board.cell(x, y) == CellState::Carved {
            return CarveResult::AlreadyCarved;
        }

        if !self.current_level().is_filled(x, y) {
            self.timer.apply_penalty(INCORRECT_CARVE_PENALTY_SECONDS);

            if self.timer.is_expired(now_ms) {
                self.timer.stop(now_ms);
                self.screen = PicrossScreen::Failed;
            }

            return CarveResult::Incorrect;
        }

        self.board.carve(x, y);

        if self.is_complete() {
            self.finish_level(now_ms);
        }

        CarveResult::Correct
    }

    /// Finish the current level and record its completion time.
    fn finish_level(&mut self, now_ms: usize) {
        self.timer.stop(now_ms);

        /*
         * elapsed_seconds() includes both real elapsed time
         * and all incorrect-carve penalties.
         */
        let completion_time = self.timer.elapsed_seconds(now_ms);

        self.completion_time_seconds = Some(completion_time);

        let mut highscores = highscores().lock();

        self.new_highscore = highscores.submit(self.selected_level, completion_time);

        self.screen = PicrossScreen::Completed;
    }

    /// Update time-dependent game state.
    pub fn update(&mut self, now_ms: usize) {
        if self.screen != PicrossScreen::Playing {
            return;
        }

        if self.timer.is_expired(now_ms) {
            self.timer.stop(now_ms);
            self.screen = PicrossScreen::Failed;
        }
    }

    /// Check whether every required solution cell has been carved.
    pub fn is_complete(&self) -> bool {
        let level = self.current_level();

        for y in 0..BOARD_SIZE {
            for x in 0..BOARD_SIZE {
                if level.is_filled(x, y) && self.board.cell(x, y) != CellState::Carved {
                    return false;
                }
            }
        }

        true
    }

    /// Select the previous level.
    ///
    /// Selection wraps around from the first level to the last.
    pub fn select_previous_level(&mut self) {
        if self.screen != PicrossScreen::LevelSelection {
            return;
        }

        if self.selected_level == 0 {
            self.selected_level = LEVELS.len() - 1;
        } else {
            self.selected_level -= 1;
        }
    }

    /// Select the next level.
    ///
    /// Selection wraps around from the last level to the first.
    pub fn select_next_level(&mut self) {
        if self.screen != PicrossScreen::LevelSelection {
            return;
        }

        self.selected_level = (self.selected_level + 1) % LEVELS.len();
    }

    /// Return to the level-selection screen.
    pub fn open_level_selection(&mut self) {
        self.screen = PicrossScreen::LevelSelection;
    }
}

impl Default for PicrossGame {
    fn default() -> Self {
        Self::new()
    }
}