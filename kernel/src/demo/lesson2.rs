/*
 * Contains a demo for heap allocations.
 *
 * Author: Michael Schoetter, Heinrich Heine University Duesseldorf
 *         Fabian Ruhland, Heinrich Heine University Duesseldorf, 2026-01-14
 * License: GPLv3
 */

use crate::allocator::global::dump_free_list;
use crate::device::key::Scancode;
use crate::device::speaker::SPEAKER;
use alloc::boxed::Box;
use alloc::vec::Vec;
use crate::device::keyboard;
use crate::device::pit::wait;

/// A simple heap demo, allocating and freeing memory on the heap.
/// The allocator state is dumped before and after each operation.
pub fn heap_demo() {
    //todo!("lesson2::heap_demo() is not implemented yet.")
    /*println!("");

    println!("Heap Demo:");
    println!("Demo 1/2: Allocate structs using 'Box'");
    println!("--------------------------------------");
    println!("");

    dump_free_list();

    #[derive(Debug)]
    struct S {
        a: usize,
        b: usize,
    }

    let s1 = Box::new(S { a: 1, b: 2 });
    let s2 = Box::new(S { a: 3, b: 4 });

    println!("");
    println!("s1 = {:?}", s1);
    println!("s2 = {:?}", s2);
    println!("");

    dump_free_list();

    println!("");
    print!("Press Enter to continue...");

    let mut s = KEYBOARD.lock();
    loop {
        let k = s.poll_key_press();
        if k.scancode() == Some(Scancode::Enter) {
            break;
        };
    }
    drop(s);

    print!("\n");
    println!("");

    println!("Heap Demo:");
    println!("Demo 2/2: Allocate a Vec of three structs");
    println!("-----------------------------------------");
    println!("");

    dump_free_list();

    let mut v: Vec<S> = Vec::new();
    v.push(S { a: 5, b: 6 });
    v.push(S { a: 7, b: 8 });
    v.push(S { a: 9, b: 0 });

    println!("");
    for i in 0..v.len() {
        println!("vec[{}] = {:?}", i, v[i]);
    }
    println!("");

    dump_free_list();

    println!("");
    print!("Press Enter to continue...");

    let mut s2 = KEYBOARD.lock();
    loop {
        let k = s2.poll_key_press();
        if k.scancode() == Some(Scancode::Enter) {
            break;
        };
    }
    drop(s2);

    print!("\n");
    println!("");*/

    let key_buffer = keyboard::keyboard_buffer();
    
    println!("");

    #[derive(Debug)]
    struct S {
        a: usize,
        b: usize,
    }

    println!("Heap Demo:");
    println!("Demo 1/4: Allocate structs using 'Box'");
    println!("--------------------------------------");
    println!("");

    // Range 0x01000000 = 16MiB
    dump_free_list();

    // Should consume 32 Bytes (16 each)
    let s1 = Box::new(S { a: 1, b: 2 });
    let s2 = Box::new(S { a: 3, b: 4 });

    println!("");
    println!("s1 = {:?}", s1);
    println!("s2 = {:?}", s2);
    println!("");

    // Compare free block size now. Should be smaller.
    dump_free_list();

    println!("");
    print!("Press Enter to continue...");
    loop {
        let k = key_buffer.poll_key_press();
        if k.scancode() == Some(Scancode::Enter) {
            break;
        };
    }

    print!("\n");
    println!("");

    println!("Demo 2/4: Free allocated structs");
    println!("--------------------------------");
    println!("");

    drop(s1);
    drop(s2);

    // See how there is now two free blocks (s1 and s2)
    dump_free_list();

    println!("");
    print!("Press Enter to continue...");
    loop {
        let k = key_buffer.poll_key_press();
        if k.scancode() == Some(Scancode::Enter) {
            break;
        };
    }

    print!("\n");
    println!("");

    println!("Demo 3/4: Allocate a Vec of three structs");
    println!("-----------------------------------------");
    println!("");

    dump_free_list();

    // Should consume 64 Bytes (16 extra if new one comes in to prevent new allocation)
    let mut v: Vec<S> = Vec::new();
    v.push(S { a: 5, b: 6 });
    v.push(S { a: 7, b: 8 });
    v.push(S { a: 9, b: 0 });

    println!("");
    for (i, item) in v.iter().enumerate() {
        println!("vec[{}] = {:?}", i, item);
    }
    println!("");

    dump_free_list();

    println!("");
    print!("Press Enter to continue...");
    loop {
        let k = key_buffer.poll_key_press();
        if k.scancode() == Some(Scancode::Enter) {
            break;
        };
    }

    print!("\n");
    println!("");

    println!("Demo 4/4: Free allocated Vec");
    println!("----------------------------");
    println!("");

    drop(v);

    // See how there is now one free block of 64 Bytes
    dump_free_list();

    println!("");
    print!("Press Enter to continue...");
    loop {
        let k = key_buffer.poll_key_press();
        if k.scancode() == Some(Scancode::Enter) {
            break;
        };
    }

    print!("\n");
    println!("");
}

/// A demo that plays songs via the PC speaker.
pub fn speaker_demo() {
    //todo!("lesson2::speaker_demo() is not implemented yet.")
    println!("This demo cannot be exited. Please reboot the system to get back to the menu.");
    let mut speaker = SPEAKER.lock();

    // Telekom Jingle (not tested yet)
    speaker.play(523, 100);
    wait(50);
    speaker.play(523, 100);
    wait(50);
    speaker.play(523, 100);
    wait(50);
    speaker.play(659, 100);
    wait(50);
    speaker.play(523, 100);

    drop(speaker);
}