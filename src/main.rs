#![no_std]
#![no_main]

use arduino_hal::{
    hal::port::PB4,
    port::{mode, Pin},
};
use panic_halt as _;

// Morse Code Specification:
// https://en.wikipedia.org/wiki/Morse_code#Representation,_timing,_and_speeds
const DURATION: u16 = 100;
const SHORT_MARK: u16 = DURATION;
const LONG_MARK: u16 = DURATION * 3;
const INTER_GAP: u16 = DURATION;
const SHORT_GAP: u16 = DURATION * 3 - INTER_GAP;
const MEDIUM_GAP: u16 = DURATION * 7 - INTER_GAP;

const MORSE_CODE: &str = "-.. . . --.. / -. ..- - ...";

fn run_morse_code(led: &mut Pin<mode::Output, PB4>) {
    MORSE_CODE.chars().for_each(|c| match c {
        '-' => {
            led.set_high();
            arduino_hal::delay_ms(LONG_MARK);
            led.set_low();
            arduino_hal::delay_ms(INTER_GAP);
        }
        '.' => {
            led.set_high();
            arduino_hal::delay_ms(SHORT_MARK);
            led.set_low();
            arduino_hal::delay_ms(INTER_GAP);
        }
        ' ' => {
            arduino_hal::delay_ms(SHORT_GAP);
        }
        '/' => {
            arduino_hal::delay_ms(MEDIUM_GAP);
        }
        _ => {}
    });
    arduino_hal::delay_ms(MEDIUM_GAP);
}

// This code works just fine
// If you are using the rust-analyzer, you might get a macro-error
// Issue thread: https://github.com/rust-analyzer/rust-analyzer/issues/10753
#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut led = pins.d12.into_output();

    loop {
        run_morse_code(&mut led);
    }
}
