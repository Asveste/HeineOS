pub mod board;
pub mod clues;
pub mod game;
pub mod input;
pub mod level;
pub mod renderer;
pub mod timer;
pub mod highscore;

pub use game::PicrossGame;
pub use level::{PicrossLevel, LEVELS};

use game::PicrossScreen;
use input::GameAction;
use crate::application::picross::input::PicrossInput;
use crate::device::key::Scancode;
use crate::device::pit;

pub const BOARD_SIZE: usize = 10;

/// Start the Picross application.
pub fn run() {
    highscore::init();

    let mut game = PicrossGame::new();
    let mut input = input::PicrossInput::new();

    let mut previous_screen = game.screen;
    let mut previous_remaining_seconds = None;

    renderer::draw(&game, pit::system_time());

    loop {
        let now_ms = pit::system_time();

        game.update(now_ms);

        let action = input.poll_action();
        let mut redraw = false;

        if let Some(action) = action {
            if handle_action(&mut game, action, now_ms, &mut input) {
                break;
            }

            redraw = true;
        }

        let remaining_seconds =
            if game.screen == PicrossScreen::Playing {
                Some(game.timer.remaining_seconds(now_ms))
            } else {
                None
            };

        if game.screen != previous_screen {
            redraw = true;
        }

        if remaining_seconds != previous_remaining_seconds {
            redraw = true;
        }

        if redraw {
            renderer::draw(&game, now_ms);

            previous_screen = game.screen;
            previous_remaining_seconds = remaining_seconds;
        }

        /*
         * Avoid spinning continuously.
         *
         * Optimized pit::wait() yields the CPU while waiting.
         */
        pit::wait(10);
    }
}

/// Apply one input action.
///
/// Returns true when the complete Picross application should close.
fn handle_action(game: &mut PicrossGame, action: GameAction, now_ms: usize, input: &mut PicrossInput) -> bool {
    match game.screen {
        PicrossScreen::LevelSelection => {
            handle_level_selection_action(game, action, now_ms, input)
        }

        PicrossScreen::Playing => {
            handle_playing_action(game, action, now_ms, input)
        }

        PicrossScreen::Completed | PicrossScreen::Failed => {
            handle_result_action(game, action, now_ms, input)
        }
    }
}

/// Handle input on the level-selection screen.
fn handle_level_selection_action(game: &mut PicrossGame, action: GameAction, now_ms: usize, input: &mut PicrossInput) -> bool {
    match action {
        GameAction::MoveUp => {
            game.select_previous_level();
        }

        GameAction::MoveDown => {
            game.select_next_level();
        }

        GameAction::CarveWithSpace => {
            game.start_level(game.selected_level, now_ms);
            input.block_until_release(Scancode::Space);
        }

        GameAction::CarveWithEnter => {
            game.start_level(game.selected_level, now_ms);
            input.block_until_release(Scancode::Enter);
        }

        GameAction::Quit => {
            return true;
        }

        _ => {}
    }

    false
}

/// Handle input while solving a puzzle.
fn handle_playing_action(game: &mut PicrossGame, action: GameAction, now_ms: usize, _input: &mut PicrossInput) -> bool {
    match action {
        GameAction::MoveUp => {
            game.move_cursor_up();
        }

        GameAction::MoveDown => {
            game.move_cursor_down();
        }

        GameAction::MoveLeft => {
            game.move_cursor_left();
        }

        GameAction::MoveRight => {
            game.move_cursor_right();
        }

        GameAction::CarveWithSpace | GameAction::CarveWithEnter => {
            game.carve_selected(now_ms);

            // Least invasive way to also still render the last cell before completion.
            // Though this can obviously be done more elegantly.
            if game.screen == PicrossScreen::Completed {
                // Temporarily draw the fully completed board.
                game.screen = PicrossScreen::Playing;
                renderer::draw(game, now_ms);

                pit::wait(1000);

                // The normal game loop will draw the completion screen next.
                game.screen = PicrossScreen::Completed;
            }
        }

        GameAction::ToggleMark => {
            game.toggle_selected_mark();
        }

        GameAction::Quit => {
            game.open_level_selection();
        }

        GameAction::Restart => {}
    }

    false
}

/// Handle input after completing or failing a puzzle.
fn handle_result_action(game: &mut PicrossGame, action: GameAction, now_ms: usize, input: &mut PicrossInput) -> bool {
    match action {
        GameAction::Restart => {
            game.start_level(game.selected_level, now_ms);
        }

        GameAction::CarveWithSpace => {
            input.block_until_release(Scancode::Space);
            game.open_level_selection();
        }

        GameAction::CarveWithEnter => {
            input.block_until_release(Scancode::Enter);
            game.open_level_selection();
        }

        GameAction::Quit => {
            return true;
        }

        _ => {}
    }

    false
}