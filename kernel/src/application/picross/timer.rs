/*
 * Countdown timer for Picross.
 */

/// Time removed after an incorrect carving attempt.
pub const INCORRECT_CARVE_PENALTY_SECONDS: usize = 120;

/// Countdown timer used while playing a Picross level.
#[derive(Clone, Copy, Debug)]
pub struct GameTimer {
    /// Initial time limit.
    limit_ms: usize,

    /// System time at which the level started.
    start_ms: usize,

    /// Additional elapsed time caused by penalties.
    penalty_ms: usize,

    /// Real elapsed time recorded when the timer stopped.
    stopped_elapsed_ms: usize,

    /// Whether the timer is currently running.
    running: bool,
}

impl GameTimer {
    /// Create an inactive timer.
    pub const fn new() -> Self {
        Self {
            limit_ms: 0,
            start_ms: 0,
            penalty_ms: 0,
            stopped_elapsed_ms: 0,
            running: false,
        }
    }

    /// Start or reset the timer.
    pub fn start(&mut self, now_ms: usize, limit_seconds: usize) {
        self.limit_ms = limit_seconds * 1000;
        self.start_ms = now_ms;
        self.penalty_ms = 0;
        self.stopped_elapsed_ms = 0;
        self.running = true;
    }

    /// Add a time penalty.
    pub fn apply_penalty(&mut self, seconds: usize) {
        self.penalty_ms = self.penalty_ms.saturating_add(seconds * 1000);
    }

    /// Stop the timer and remember the elapsed real time.
    pub fn stop(&mut self, now_ms: usize) {
        if !self.running {
            return;
        }

        self.stopped_elapsed_ms = now_ms.saturating_sub(self.start_ms);

        self.running = false;
    }

    /// Get elapsed time, including penalties.
    pub fn elapsed_ms(&self, now_ms: usize) -> usize {
        let real_elapsed = if self.running {
            now_ms.saturating_sub(self.start_ms)
        } else {
            self.stopped_elapsed_ms
        };

        real_elapsed.saturating_add(self.penalty_ms)
    }

    /// Get the remaining countdown time.
    pub fn remaining_ms(&self, now_ms: usize) -> usize {
        self.limit_ms.saturating_sub(self.elapsed_ms(now_ms))
    }

    /// Get the remaining time rounded up to complete seconds.
    ///
    /// For example, 4,001 milliseconds are displayed as 5 seconds.
    pub fn remaining_seconds(&self, now_ms: usize) -> usize {
        let remaining = self.remaining_ms(now_ms);

        if remaining == 0 {
            0
        } else {
            (remaining + 999) / 1000
        }
    }

    /// Get elapsed time rounded up to complete seconds.
    pub fn elapsed_seconds(&self, now_ms: usize) -> usize {
        let elapsed = self.elapsed_ms(now_ms);

        if elapsed == 0 {
            0
        } else {
            (elapsed + 999) / 1000
        }
    }

    /// Check whether no time remains.
    pub fn is_expired(&self, now_ms: usize) -> bool {
        self.remaining_ms(now_ms) == 0
    }

    pub fn is_running(&self) -> bool {
        self.running
    }
}

impl Default for GameTimer {
    fn default() -> Self {
        Self::new()
    }
}