/*
 * Contains demos for text output and keyboard input.
 *
 * Author: Michael Schoetter, Heinrich Heine University Duesseldorf
 *         Fabian Ruhland, Heinrich Heine University Duesseldorf, 2026-01-14
 * License: GPLv3
 */
use crate::device::key::Scancode;
use crate::device::keyboard::KEYBOARD;

/// A simple text demo, displaying formatted numbers.
pub fn text_demo() {
    //todo!("lesson1::text_demo() not implemented yet");
    let n = 16;

    println!("Text Demo:");
    println!("|{:>4} |{:>4} |{:>7} |", "dec", "hex", "bin");
    println!("|{:->5}|{:->5}|{:->8}|", "", "", "");

    for i in 0..=n {
        println!("|{:>4} |{:>4x} |{:>7b} |", i, i, i);
    }
}

/// A simple keyboard demo, displaying the events of key presses and releases.
pub fn keyboard_demo() {
    //todo!("lesson1::keyboard_demo() not implemented yet");
    println!("Keyboard Demo:");
    println!("Press keys on your keyboard. Press 'ESC' to exit the demo.");
    let mut s = KEYBOARD.lock();
    loop {
        let k = s.poll_key_event();
        if k.scancode() == Some(Scancode::Escape) {
            break;
        };
        println!("Key: {:?}", k);
    }
}