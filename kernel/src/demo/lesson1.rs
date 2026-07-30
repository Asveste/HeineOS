/*
 * Contains demos for text output and keyboard input.
 *
 * Author: Michael Schoetter, Heinrich Heine University Duesseldorf
 *         Fabian Ruhland, Heinrich Heine University Duesseldorf, 2026-01-14
 * License: GPLv3
 */
use crate::device::key::Scancode;
use crate::device::keyboard;
use crate::device::terminal::{framebuffer, terminal};

/// A simple text demo, displaying formatted numbers.
pub fn text_demo() {
    //todo!("lesson1::text_demo() not implemented yet");
    let mut n = 16;

    println!("Text Demo:");
    println!("Showing 64 numbers. Press S to continue.\n");
    println!("|{:>4} |{:>4} |{:>7} |", "dec", "hex", "bin");
    println!("|{:->5}|{:->5}|{:->8}|", "", "", "");

    let key_buffer = keyboard::keyboard_buffer();
    while n <= 64 {
        let key = key_buffer.poll_key_press();

        if key.scancode() == Some(Scancode::S) && n <= 64 {
            for i in 0..=n {
                println!("|{:>4} |{:>4x} |{:>7b} |", i, i, i);
            }
            n += 16;
        }
    }
}

/// A simple keyboard demo, displaying the events of key presses and releases.
pub fn keyboard_demo() {
    //todo!("lesson1::keyboard_demo() not implemented yet");
    println!("Keyboard Demo:");
    println!("Press keys on your keyboard. Press 'ESC' to exit the demo.\n");

    let key_buffer = keyboard::keyboard_buffer();

    loop {
        let key = key_buffer.poll_key_event();

        if key.pressed() && key.scancode() == Some(Scancode::Escape) {
            println!("Exiting demo.");
            break;
        }

        println!("Key: {:?}", key);
    }
}