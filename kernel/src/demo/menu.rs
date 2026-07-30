/*
 * Menu for selecting and running the different kernel demos.
 */
use crate::application::picross;
use crate::demo::lesson1::{keyboard_demo, text_demo};
use crate::demo::lesson2::{heap_demo, speaker_demo};
use crate::demo::lesson4::{coroutine_demo, thread_demo};
use crate::demo::lesson6::peanut_gb;
use crate::device::pit::wait;
use crate::device::terminal::terminal;
use crate::library::bitmap;
use crate::library::input::read_char;
use crate::thread::scheduler::scheduler;
use crate::thread::thread::Thread;

/// Clear the terminal screen.
///
/// The terminal lock is released when this function returns.
fn clear_screen() {
    let mut terminal = terminal().lock();
    terminal.clear();
}

/// Print all available demos.
fn draw_menu() {
    clear_screen();

    // Fanart by Lilith_Kiwi
    // Source: https://www.artstation.com/artwork/8wJ4Nq
    let bm = bitmap::Bitmap::read_from_file("kirby.bmp")
        .expect("Failed to read bitmap file")
        .expect("Invalid or unsupported bitmap");

    terminal().lock().draw_bitmap_centered(&bm);

    println!(" -----------");
    println!(" |Demo Menu|");
    println!(" -----------\n");
    println!("  1. Text Demo");
    println!("  2. Keyboard Demo");
    println!("  3. Heap Demo");
    println!("  4. PC Speaker Demo");
    println!("  5. Coroutine Demo");
    println!("  6. Thread Demo");
    println!("  7. Peanut-GB Demo");
    println!("  8. Picross Demo\n");

    print!("Select a demo by pressing one\n of the listed numbers: ");
}

/// Wait until the user presses any key.
fn wait_for_any_key() {
    println!("");
    println!("Press any key to return to the menu...");

    let _ = read_char();
}

/// Run the demo menu forever.
pub fn run() -> ! {
    loop {
        draw_menu();

        let selection = read_char();

        // Echo the selected character.
        print!("{}", selection);
        wait(600);

        match selection {
            '1' => {
                clear_screen();
                text_demo();
                wait_for_any_key();
            }

            '2' => {
                clear_screen();
                keyboard_demo();
                wait_for_any_key();
            }

            '3' => {
                clear_screen();
                heap_demo();
                wait_for_any_key();
            }

            '4' => {
                clear_screen();
                let one = Thread::new(speaker_demo);
                scheduler().ready(one);
                scheduler().schedule();
                //wait_for_any_key();
            }

            '5' => {
                clear_screen();
                coroutine_demo();
                wait_for_any_key();
            }

            '6' => {
                clear_screen();
                thread_demo();
                wait_for_any_key();
            }

            '7' => {
                clear_screen();
                peanut_gb::play("roms/2048.gb");
                wait_for_any_key();
            }

            '8' => {
                clear_screen();
                picross::run();
                clear_screen();
                //wait_for_any_key();
            }

            _ => {
                println!("Invalid selection.");
                wait(300);
            }
        }
    }
}