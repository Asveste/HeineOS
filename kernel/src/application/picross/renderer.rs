/*
 * Rendering functions for the Picross game.
 */

use super::board::CellState;
use super::game::{PicrossGame, PicrossScreen};
use super::clues::MAX_CLUES;
use super::BOARD_SIZE;
use super::level::LEVELS;
use super::highscore::highscores;

use crate::device::framebuffer::{self, Framebuffer};
use crate::device::terminal::framebuffer as global_framebuffer;

/// Width and height of one Picross cell in framebuffer pixels.
const CELL_SIZE: usize = 24;

/// Total width and height of the 10x10 board, including the final grid line.
const BOARD_PIXEL_SIZE: usize = BOARD_SIZE * CELL_SIZE + 1;

/// Space reserved to the left and above the board for clue numbers.
const CLUE_AREA_SIZE: usize = MAX_CLUES * CELL_SIZE;

/// Draw the current Picross screen.
pub fn draw(game: &PicrossGame, now_ms: usize) {
    let mut framebuffer = global_framebuffer().lock();

    framebuffer.clear();

    match game.screen {
        PicrossScreen::LevelSelection => {
            draw_level_selection(&mut framebuffer, game);
        }
        PicrossScreen::Playing => {
            draw_playing_screen(&mut framebuffer, game, now_ms);
        }
        PicrossScreen::Completed => {
            draw_completed_screen(&mut framebuffer, game);
        }
        PicrossScreen::Failed => {
            draw_failed_screen(&mut framebuffer);
        }
    }
}

/// Draw the level-selection screen.
fn draw_level_selection(framebuffer: &mut Framebuffer, game: &PicrossGame) {
    let title = "PICROSS";

    let title_width = title.len() * framebuffer::CHAR_WIDTH;

    let title_x = framebuffer.width().saturating_sub(title_width) / 2;

    framebuffer.draw_str(title, title_x, 40, framebuffer::GREEN, framebuffer::BLACK);

    let subtitle = "SELECT LEVEL";

    let subtitle_width = subtitle.len() * framebuffer::CHAR_WIDTH;

    let subtitle_x = framebuffer.width().saturating_sub(subtitle_width) / 2;

    framebuffer.draw_str(subtitle, subtitle_x, 80, framebuffer::WHITE, framebuffer::BLACK);

    draw_level_list(framebuffer, game);

    draw_level_selection_controls(framebuffer);
}

/// Draw controls for the level-selection screen.
fn draw_level_selection_controls(framebuffer: &mut Framebuffer) {
    let first_line_y = framebuffer.height().saturating_sub(56);

    framebuffer.draw_str(
        "W/S: Select   ENTER/SPACE: Start",
        16,
        first_line_y,
        framebuffer::WHITE,
        framebuffer::BLACK,
    );

    framebuffer.draw_str(
        "Q: Exit",
        16,
        first_line_y + framebuffer::CHAR_HEIGHT + 4,
        framebuffer::WHITE,
        framebuffer::BLACK,
    );
}

/// Draw all available levels and their best times.
fn draw_level_list(framebuffer: &mut Framebuffer, game: &PicrossGame) {
    const LIST_WIDTH: usize = 360;
    const FIRST_LEVEL_Y: usize = 130;
    const LEVEL_DISTANCE: usize = 32;

    const NAME_OFFSET: usize = 24;
    const BEST_LABEL_OFFSET: usize = 176;
    const BEST_TIME_OFFSET: usize = 232;

    let list_x = framebuffer.width().saturating_sub(LIST_WIDTH) / 2;

    let highscores = highscores().lock();

    for index in 0..LEVELS.len() {
        let y = FIRST_LEVEL_Y + index * LEVEL_DISTANCE;

        let selected = index == game.selected_level;

        let color = if selected {
            framebuffer::CYAN
        } else {
            framebuffer::WHITE
        };

        if selected {
            framebuffer.draw_str(">", list_x, y, framebuffer::CYAN, framebuffer::BLACK);
        }

        framebuffer.draw_str(LEVELS[index].name(), list_x + NAME_OFFSET, y, color, framebuffer::BLACK);

        framebuffer.draw_str("BEST:", list_x + BEST_LABEL_OFFSET, y, color, framebuffer::BLACK);

        match highscores.best(index) {
            Some(seconds) => {
                draw_seconds_value(framebuffer, seconds, list_x + BEST_TIME_OFFSET, y, color);
            }

            None => {
                framebuffer.draw_str("--:--", list_x + BEST_TIME_OFFSET, y, color, framebuffer::BLACK);
            }
        }
    }
}

fn draw_playing_screen(framebuffer: &mut Framebuffer, game: &PicrossGame, now_ms: usize, ) {
    framebuffer.draw_str("PICROSS", 16, 16, framebuffer::GREEN, framebuffer::BLACK);

    framebuffer.draw_str(game.current_level().name(), 16, 40, framebuffer::GREEN, framebuffer::BLACK);

    draw_timer(framebuffer, game.timer.remaining_seconds(now_ms));

    let board_x = framebuffer.width().saturating_sub(BOARD_PIXEL_SIZE) / 2;

    let board_y = framebuffer.height().saturating_sub(BOARD_PIXEL_SIZE) / 2;

    draw_row_clues(framebuffer, game, board_x, board_y);

    draw_column_clues(framebuffer, game, board_x, board_y);

    draw_board(framebuffer, game, board_x, board_y);

    draw_controls(framebuffer);
}

/// Draw the current keyboard controls.
fn draw_controls(framebuffer: &mut Framebuffer) {
    let controls_y = framebuffer.height().saturating_sub(56);

    framebuffer.draw_str(
        "WASD: Move",
        16,
        controls_y,
        framebuffer::GREEN,
        framebuffer::BLACK,
    );

    framebuffer.draw_str(
        "SPACE: Carve   X: Mark   Q: Levels",
        16,
        controls_y + framebuffer::CHAR_HEIGHT + 4,
        framebuffer::GREEN,
        framebuffer::BLACK,
    );
}

/// Draw the board, its cells, the grid, and the cursor.
fn draw_board(framebuffer: &mut Framebuffer, game: &PicrossGame, board_x: usize, board_y: usize) {
    draw_cells(framebuffer, game, board_x, board_y);

    draw_grid(framebuffer, board_x, board_y);

    draw_cursor(framebuffer, game, board_x, board_y);
}

/// Draw the countdown in MM:SS format.
fn draw_timer(framebuffer: &mut Framebuffer, total_seconds: usize) {
    let timer_width = 10 * framebuffer::CHAR_WIDTH;

    let x = framebuffer.width().saturating_sub(timer_width + 16);

    let y = 16;

    framebuffer.draw_str(
        "TIME ",
        x,
        y,
        framebuffer::WHITE,
        framebuffer::BLACK,
    );

    draw_seconds_value(
        framebuffer,
        total_seconds,
        x + 5 * framebuffer::CHAR_WIDTH,
        y,
        framebuffer::WHITE,
    );
}

/// Draw MM:SS.
fn draw_time_value(framebuffer: &mut Framebuffer, minutes: usize, seconds: usize, x: usize, y: usize, color: u32) {
    let characters = [
        digit_char((minutes / 10) % 10),
        digit_char(minutes % 10),
        ':',
        digit_char((seconds / 10) % 10),
        digit_char(seconds % 10),
    ];

    for (index, character) in characters.iter().enumerate() {
        framebuffer.draw_char(
            *character,
            x + index * framebuffer::CHAR_WIDTH,
            y,
            color,
            framebuffer::BLACK,
        );
    }
}

/// Draw a number of seconds in MM:SS format.
fn draw_seconds_value(framebuffer: &mut Framebuffer, total_seconds: usize, x: usize, y: usize, color: u32) {
    let minutes = (total_seconds / 60).min(99);
    let seconds = total_seconds % 60;

    draw_time_value(framebuffer, minutes, seconds, x, y, color);
}

/// Convert one decimal digit into its character.
fn digit_char(digit: usize) -> char {
    match digit {
        0 => '0',
        1 => '1',
        2 => '2',
        3 => '3',
        4 => '4',
        5 => '5',
        6 => '6',
        7 => '7',
        8 => '8',
        9 => '9',
        _ => '?',
    }
}

/// Draw the contents of all 100 cells.
fn draw_cells(framebuffer: &mut Framebuffer, game: &PicrossGame, board_x: usize, board_y: usize) {
    for y in 0..BOARD_SIZE {
        for x in 0..BOARD_SIZE {
            let cell_x = board_x + x * CELL_SIZE;
            let cell_y = board_y + y * CELL_SIZE;

            match game.board.cell(x, y) {
                CellState::Uncarved => {
                    draw_uncovered_cell(framebuffer, cell_x, cell_y);
                }

                CellState::Marked => {
                    draw_uncovered_cell(framebuffer, cell_x, cell_y);
                    draw_mark(framebuffer, cell_x, cell_y);
                }

                CellState::Carved => {
                    draw_carved_cell(framebuffer, cell_x, cell_y);
                }
            }
        }
    }
}

/// Draw an uncarved cell.
fn draw_uncovered_cell(framebuffer: &mut Framebuffer, x: usize, y: usize) {
    fill_rectangle(
        framebuffer,
        x + 1,
        y + 1,
        CELL_SIZE - 1,
        CELL_SIZE - 1,
        framebuffer::BLACK,
    );
}

/// Draw a successfully carved cell.
fn draw_carved_cell(framebuffer: &mut Framebuffer, x: usize, y: usize) {
    fill_rectangle(
        framebuffer,
        x + 1,
        y + 1,
        CELL_SIZE - 1,
        CELL_SIZE - 1,
        framebuffer::WHITE,
    );
}

/// Draw an X inside a marked cell.
fn draw_mark(framebuffer: &mut Framebuffer, cell_x: usize, cell_y: usize) {
    const MARGIN: usize = 5;

    for offset in MARGIN..=(CELL_SIZE - MARGIN) {
        framebuffer.draw_pixel(
            cell_x + offset,
            cell_y + offset,
            framebuffer::YELLOW,
        );

        framebuffer.draw_pixel(
            cell_x + CELL_SIZE - offset,
            cell_y + offset,
            framebuffer::YELLOW,
        );
    }
}

/// Draw the complete 10x10 grid.
fn draw_grid(framebuffer: &mut Framebuffer, board_x: usize, board_y: usize) {
    for index in 0..=BOARD_SIZE {
        let line_x = board_x + index * CELL_SIZE;
        let line_y = board_y + index * CELL_SIZE;

        draw_vertical_line(
            framebuffer,
            line_x,
            board_y,
            BOARD_PIXEL_SIZE,
            framebuffer::WHITE,
        );

        draw_horizontal_line(
            framebuffer,
            board_x,
            line_y,
            BOARD_PIXEL_SIZE,
            framebuffer::WHITE,
        );
    }
}

/// Draw a hollow square around the selected cell.
fn draw_cursor(framebuffer: &mut Framebuffer, game: &PicrossGame, board_x: usize, board_y: usize) {
    let cursor_x = board_x + game.cursor.x * CELL_SIZE;
    let cursor_y = board_y + game.cursor.y * CELL_SIZE;

    draw_rectangle(
        framebuffer,
        cursor_x + 3,
        cursor_y + 3,
        CELL_SIZE - 5,
        CELL_SIZE - 5,
        framebuffer::CYAN,
    );
}

/// Draw the completion screen and final time.
fn draw_completed_screen(framebuffer: &mut Framebuffer, game: &PicrossGame, ) {
    let title = "PUZZLE COMPLETED!";

    let title_width = title.len() * framebuffer::CHAR_WIDTH;

    let title_x = framebuffer.width().saturating_sub(title_width) / 2;

    let start_y = framebuffer.height().saturating_sub(220) / 2;

    framebuffer.draw_str(
        title, 
        title_x, 
        start_y, 
        framebuffer::GREEN, 
        framebuffer::BLACK
    );

    const INFO_WIDTH: usize = 13 * framebuffer::CHAR_WIDTH;

    let info_x = framebuffer.width().saturating_sub(INFO_WIDTH) / 2;

    framebuffer.draw_str(
        "TIME:", 
        info_x, 
        start_y + 40, 
        framebuffer::WHITE, 
        framebuffer::BLACK
    );

    if let Some(seconds) = game.completion_time_seconds {
        draw_seconds_value(framebuffer, seconds, info_x + 7 * framebuffer::CHAR_WIDTH, start_y + 40, framebuffer::WHITE);
    }

    framebuffer.draw_str(
        "BEST:", 
        info_x, 
        start_y + 64, 
        framebuffer::WHITE, 
        framebuffer::BLACK
    );

    let best_time = {
        let highscores = highscores().lock();
        highscores.best(game.selected_level)
    };

    if let Some(seconds) = best_time {
        draw_seconds_value(framebuffer, seconds, info_x + 7 * framebuffer::CHAR_WIDTH, start_y + 64, framebuffer::WHITE);
    }

    if game.new_highscore {
        let message = "NEW BEST TIME!";

        let message_width = message.len() * framebuffer::CHAR_WIDTH;

        let message_x = framebuffer.width().saturating_sub(message_width) / 2;

        framebuffer.draw_str(
            message,
            message_x,
            start_y + 96,
            framebuffer::YELLOW,
            framebuffer::BLACK,
        );
    }

    framebuffer.draw_str(
        "R: Restart",
        info_x,
        start_y + 136,
        framebuffer::WHITE,
        framebuffer::BLACK,
    );

    framebuffer.draw_str(
        "ENTER/SPACE: Levels",
        info_x,
        start_y + 160,
        framebuffer::WHITE,
        framebuffer::BLACK,
    );

    framebuffer.draw_str(
        "Q: Exit",
        info_x,
        start_y + 184,
        framebuffer::WHITE,
        framebuffer::BLACK,
    );
}

/// Draw the failure screen.
fn draw_failed_screen(framebuffer: &mut Framebuffer) {
    let title = "TIME EXPIRED!";

    let title_width = title.len() * framebuffer::CHAR_WIDTH;

    let title_x = framebuffer.width().saturating_sub(title_width) / 2;

    let title_y = framebuffer.height().saturating_sub(framebuffer::CHAR_HEIGHT) / 2;

    framebuffer.draw_str(
        title,
        title_x,
        title_y,
        framebuffer::RED,
        framebuffer::BLACK,
    );

    framebuffer.draw_str(
        "R: Restart",
        title_x,
        title_y + 32,
        framebuffer::WHITE,
        framebuffer::BLACK,
    );

    framebuffer.draw_str(
        "Q: Quit",
        title_x,
        title_y + 52,
        framebuffer::WHITE,
        framebuffer::BLACK,
    );
}

/// Fill a rectangular area with one color.
fn fill_rectangle(framebuffer: &mut Framebuffer, x: usize, y: usize, width: usize, height: usize, color: u32) {
    for y_offset in 0..height {
        for x_offset in 0..width {
            framebuffer.draw_pixel(x + x_offset, y + y_offset, color);
        }
    }
}

/// Draw the outline of a rectangle.
fn draw_rectangle(framebuffer: &mut Framebuffer, x: usize, y: usize, width: usize, height: usize, color: u32) {
    if width == 0 || height == 0 {
        return;
    }

    draw_horizontal_line(framebuffer, x, y, width, color);

    draw_horizontal_line(framebuffer, x, y + height - 1, width, color);

    draw_vertical_line(framebuffer, x, y, height, color);

    draw_vertical_line(framebuffer, x + width - 1, y, height, color);
}

/// Draw a horizontal line.
fn draw_horizontal_line(framebuffer: &mut Framebuffer, x: usize, y: usize, length: usize, color: u32) {
    for offset in 0..length {
        framebuffer.draw_pixel(x + offset, y, color);
    }
}

/// Draw a vertical line.
fn draw_vertical_line(framebuffer: &mut Framebuffer, x: usize, y: usize, length: usize, color: u32) {
    for offset in 0..length {
        framebuffer.draw_pixel(x, y + offset, color);
    }
}

/// Draw the clues to the left of every row.
fn draw_row_clues(framebuffer: &mut Framebuffer, game: &PicrossGame, board_x: usize, board_y: usize) {
    for row in 0..BOARD_SIZE {
        let clue = &game.clues.rows[row];

        /*
         * Right-align the clue values.
         *
         * A clue containing [3, 2] is placed into the last two
         * available clue slots next to the board.
         */
        let first_slot = MAX_CLUES - clue.len();

        for index in 0..clue.len() {
            let slot = first_slot + index;

            let cell_x = board_x - CLUE_AREA_SIZE + slot * CELL_SIZE;

            let cell_y = board_y + row * CELL_SIZE;

            draw_clue_number(framebuffer, clue.value(index), cell_x, cell_y);
        }
    }
}

/// Draw the clues above every column.
fn draw_column_clues(framebuffer: &mut Framebuffer, game: &PicrossGame, board_x: usize, board_y: usize) {
    for column in 0..BOARD_SIZE {
        let clue = &game.clues.columns[column];

        /*
         * Bottom-align the clue values so the last value is
         * directly above the board.
         */
        let first_slot = MAX_CLUES - clue.len();

        for index in 0..clue.len() {
            let slot = first_slot + index;

            let cell_x = board_x + column * CELL_SIZE;

            let cell_y = board_y - CLUE_AREA_SIZE + slot * CELL_SIZE;

            draw_clue_number(framebuffer, clue.value(index), cell_x, cell_y);
        }
    }
}

/// Draw a clue number centered inside an imaginary CELL_SIZE square.
fn draw_clue_number(framebuffer: &mut Framebuffer, value: u8, cell_x: usize, cell_y: usize) {
    let text = clue_number_text(value);

    let text_width = text.len() * framebuffer::CHAR_WIDTH;

    let text_x = cell_x + CELL_SIZE.saturating_sub(text_width) / 2;

    let text_y = cell_y + CELL_SIZE.saturating_sub(framebuffer::CHAR_HEIGHT) / 2;

    framebuffer.draw_str(text, text_x, text_y, framebuffer::WHITE, framebuffer::BLACK);
}

/// Convert a possible clue value into static text.
fn clue_number_text(value: u8) -> &'static str {
    match value {
        0 => "0",
        1 => "1",
        2 => "2",
        3 => "3",
        4 => "4",
        5 => "5",
        6 => "6",
        7 => "7",
        8 => "8",
        9 => "9",
        10 => "10",
        _ => "?",
    }
}