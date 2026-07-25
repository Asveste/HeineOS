/*
 * A driver for the programmable interval timer (PIT).
 *
 * Author: Michael Schoettner, Heinrich Heine University Duesseldorf, 2023-06-15
 *         Fabian Ruhland, Heinrich Heine University Duesseldorf, 2026-01-15
 * License: GPLv3
 */

use alloc::boxed::Box;
use core::sync::atomic::AtomicUsize;
use crate::device::cpu::IoPort;
use crate::device::pic::{Irq, PIC};
use crate::device::{font_8x8, framebuffer, terminal};
use crate::interrupt;
use crate::interrupt::dispatcher::{IntVectors, InterruptVector};
use crate::interrupt::isr::ISR;
use crate::library::once::Once;
use crate::thread::scheduler::scheduler;

/// Get the current system time in milliseconds.
pub fn system_time() -> usize {
    SYSTEM_TIME.load(core::sync::atomic::Ordering::Relaxed)
}

/// Wait for a specified number of milliseconds using the system time.
pub fn wait(ms: usize) {
    let start_time = system_time();
    while system_time() - start_time < ms {
        scheduler().yield_cpu();
    }
}

#[repr(u16)]
/// I/O port addresses for the PIT.
enum PitRegister {
    Control = 0x43,
    Data = 0x40
}

/// Frequency of the timer in Hz
const TIMER_FREQUENCY: usize = 1193182;

/// Nanoseconds that pass per timer tick
const NANOSECONDS_PER_TICK: usize = 1_000_000_000 / TIMER_FREQUENCY;

/// The interval at which the timer should generate interrupts (1 ms).
const TIMER_INTERRUPT_INTERVAL_MS: usize = 1;

/// The interval at which the spinner should change to the next state
const SPINNER_INTERVAL_MS: usize = 250;

/// Global timer instance
static TIMER: Once<Timer> = Once::new();

/// System time in milliseconds.
/// This variable is updated by the timer interrupt service routine.
static SYSTEM_TIME: AtomicUsize = AtomicUsize::new(0);

/// Characters used for the spinner animation.
static SPINNER_CHARS: &[char] = &['|', '/', '-', '\\'];

/// Register the timer interrupt handler.
pub fn plugin() {
    //todo!("pit::plugin() is not yet implemented");
    TIMER.init(|| {
        let mut timer = Timer::new();
        timer.set_interrupt_interval(TIMER_INTERRUPT_INTERVAL_MS);
        timer
    });

    IntVectors::register(
        InterruptVector::Pit,
        Box::new(TimerISR {
            interval_ms: TIMER_INTERRUPT_INTERVAL_MS
        })
    );

    let mut s = PIC.lock();
    s.allow(Irq::Timer);
}

/// Represents the programmable interval timer.
struct Timer {
    control_port: IoPort,
    data_port0: IoPort
}

/// The timer interrupt service routine.
struct TimerISR {
    interval_ms: usize,
}

impl ISR for TimerISR {
    /// Handle the timer interrupt.
    /// This function updates the system time and triggers a context switch every 10 ms.
    fn trigger(&self) {
        //todo!("pit::trigger() is not yet implemented");
        let current_time = SYSTEM_TIME.fetch_add(
            self.interval_ms,
            core::sync::atomic::Ordering::Relaxed,
        ) + self.interval_ms;

        const SPINNER_INTERVAL_MS: usize = 250;

        if current_time % SPINNER_INTERVAL_MS != 0 {
            return;
        }

        let spinner_step = current_time / SPINNER_INTERVAL_MS;
        let spinner_index = (spinner_step - 1) % SPINNER_CHARS.len();
        let spinner_char = SPINNER_CHARS[spinner_index];

        if let Some(mut framebuffer) = terminal::framebuffer().try_lock() {
            let x = framebuffer
                .width()
                .saturating_sub(font_8x8::CHAR_WIDTH);

            let y = 0;

            framebuffer.draw_char(
                spinner_char,
                x,
                y,
                framebuffer::WHITE,
                framebuffer::BLACK,
            );
        }
    }
}

impl Timer {
    /// Create a new Timer instance.
    pub const fn new() -> Timer {
        Timer {
            control_port: IoPort::new(PitRegister::Control as u16),
            data_port0: IoPort::new(PitRegister::Data as u16)
        }
    }

    /// Set the timer interrupt interval in milliseconds.
    pub fn set_interrupt_interval(&mut self, interval_ms: usize) {
        //todo!("pit::set_interrupt_interval() is not yet implemented");
        let counter_value = (TIMER_FREQUENCY * interval_ms) / 1000;

        assert!(
            counter_value > 0 && counter_value <= u16::MAX as usize,
            "PIT interval is out of range"
        );

        let counter_value = counter_value as u16;

        let low_byte = counter_value as u8;
        let high_byte = (counter_value >> 8) as u8;

        unsafe {
            self.control_port.outb(0x36);
            self.data_port0.outb(low_byte);
            self.data_port0.outb(high_byte);
        }
    }
}
