/*
 * Keyboard input handling for Picross.
 */

use crate::device::key::Scancode;
use crate::device::keyboard::keyboard_buffer;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GameAction {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    CarveWithSpace,
    CarveWithEnter,
    ToggleMark,
    Restart,
    Quit,
}

pub struct PicrossInput {
    /// A key that must be released before it can produce another action.
    blocked_key: Option<Scancode>,
}

impl PicrossInput {
    pub const fn new() -> Self {
        Self {
            blocked_key: None,
        }
    }

    /// Prevent the given key from immediately triggering another action.
    pub fn block_until_release(&mut self, scancode: Scancode) {
        self.blocked_key = Some(scancode);
    }

    /// Check for an action without blocking.
    pub fn poll_action(&mut self) -> Option<GameAction> {
        loop {
            let key = keyboard_buffer().pop_key_event()?;
            let scancode = key.scancode()?;

            /*
             * If this key is blocked, ignore it until its release
             * event arrives.
             */
            if self.blocked_key == Some(scancode) {
                if !key.pressed() {
                    self.blocked_key = None;
                }

                continue;
            }

            // Ignore all other key-release events.
            if !key.pressed() {
                continue;
            }

            return match scancode {
                Scancode::W | Scancode::Up => {
                    Some(GameAction::MoveUp)
                }

                Scancode::S | Scancode::Down => {
                    Some(GameAction::MoveDown)
                }

                Scancode::A | Scancode::Left => {
                    Some(GameAction::MoveLeft)
                }

                Scancode::D | Scancode::Right => {
                    Some(GameAction::MoveRight)
                }

                Scancode::Enter => {
                    Some(GameAction::CarveWithEnter)
                }

                Scancode::Space => {
                    Some(GameAction::CarveWithSpace)
                }

                Scancode::X => {
                    Some(GameAction::ToggleMark)
                }

                Scancode::R => {
                    Some(GameAction::Restart)
                }

                Scancode::Q | Scancode::Escape => {
                    Some(GameAction::Quit)
                }

                _ => None,
            };
        }
    }
}