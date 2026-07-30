/*
 * In-memory highscore storage for Picross.
 */

use super::level::LEVEL_COUNT;

use crate::library::once::Once;
use crate::library::spinlock::Spinlock;

/// Global highscore storage.
///
/// Scores survive leaving and reopening Picross, but disappear after reboot.
static HIGHSCORES: Once<Spinlock<Highscores>> = Once::new();

/// Best completion time for every level.
///
/// A value of `None` means the level has not been completed yet.
pub struct Highscores {
    times: [Option<usize>; LEVEL_COUNT],
}

impl Highscores {
    pub const fn new() -> Self {
        Self {
            times: [None; LEVEL_COUNT],
        }
    }

    /// Get the best completion time for one level.
    pub fn best(&self, level: usize) -> Option<usize> {
        self.times.get(level).copied().flatten()
    }

    /// Submit a completion time.
    ///
    /// Returns `true` when this is a new highscore.
    pub fn submit(&mut self, level: usize, completion_seconds: usize) -> bool {
        let Some(current_best) = self.times.get_mut(level) else {
            return false;
        };

        match *current_best {
            None => {
                *current_best = Some(completion_seconds);
                true
            }

            Some(previous_best)
            if completion_seconds < previous_best => {
                *current_best = Some(completion_seconds);
                true
            }

            Some(_) => false,
        }
    }
}

impl Default for Highscores {
    fn default() -> Self {
        Self::new()
    }
}

/// Initialize the global highscore storage.
///
/// Calling this again after initialization does nothing.
pub fn init() {
    if HIGHSCORES.get().is_none() {
        HIGHSCORES.init(|| {
            Spinlock::new(Highscores::new())
        });
    }
}

/// Access the global highscore storage.
pub fn highscores() -> &'static Spinlock<Highscores> {
    HIGHSCORES.get().expect("Picross highscores not initialized")
}