/*
 * Contains demos for coroutines and threads.
 *
 * Author: Michael Schoetter, Heinrich Heine University Duesseldorf
 *         Fabian Ruhland, Heinrich Heine University Duesseldorf, 2026-01-15
 * License: GPLv3
 */
use log::info;
use crate::coroutine::coroutine::Coroutine;
use crate::device::pit::wait;
use crate::device::terminal::{terminal, Terminal};
use crate::thread::scheduler;
use crate::thread::scheduler::scheduler;
use crate::thread::thread::Thread;

/// A demo function showcasing coroutines.
/// It starts three coroutines, each incrementing a counter and printing it to the terminal in an endless loop.
/// The coroutines switch to the next coroutine after each print.
pub fn coroutine_demo() {
    //todo!("lesson4::coroutine_demo() is not implemented yet.");
    println!("Coroutine Demo:");
    println!("This demo cannot be exited. Please reboot the system to get back to the menu.");

    let mut one = Coroutine::new(coroutine_loop);
    let mut two = Coroutine::new(coroutine_loop);
    let mut three = Coroutine::new(coroutine_loop);

    one.set_next(&mut two);
    two.set_next(&mut three);
    three.set_next(&mut one);

    one.start();
}

/// The function executed by each coroutine in the coroutine demo.
/// It increments a counter and prints it to the terminal in an endless loop,
/// switching to the next coroutine after each print.
fn coroutine_loop(coroutine: &mut Coroutine) {
    //todo!("lesson4::coroutine_loop() is not implemented yet.");
    let mut count = 0;
    loop {
        count += 1;
        {
            let mut t = terminal().lock();
            t.set_pos(1, 4 + coroutine.id());
            print_terminal!(&mut *t, "Coroutine [{}]: {}", coroutine.id(), count);
        }
        coroutine.switch();
    }
}

/// A demo function showcasing threads.
/// It starts three threads, each incrementing a counter and printing it to the terminal in an endless loop.
/// The threads yield the CPU to the next thread after each print.
/// The first thread also kills the other two threads after a certain number of iterations and finally exits itself, ending the demo.
pub fn thread_demo() {
    //todo!("lesson4::thread_demo() is not implemented yet.");
    println!("Thread Demo:");
    println!("This demo cannot be exited. Please reboot the system to get back to the menu.");

    let one = Thread::new(thread_entry);
    let two = Thread::new(thread_entry);
    let three = Thread::new(thread_entry);

    scheduler().ready(one);
    scheduler().ready(two);
    scheduler().ready(three);

    scheduler().schedule();
}

/// The function executed by each thread in the thread demo.
/// It increments a counter and prints it to the terminal in an endless loop,
/// yielding the CPU to the next thread after each print.
fn thread_entry() {
    //todo!("lesson4::thread_entry() is not implemented yet.");
    let mut count = 0;

    loop {
        let tid = scheduler().get_active_tid();

        count += 1;
        {
            let mut t = terminal().lock();
            t.set_pos(1, 4 + scheduler().get_active_tid());
            print_terminal!(&mut *t, "Thread [{}]: {}", tid, count);
        }
        wait(100);

        if count >= 333 {
            scheduler().exit();
        }
    }
}