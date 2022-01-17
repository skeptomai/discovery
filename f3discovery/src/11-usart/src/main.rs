#![no_main]
#![no_std]

#[allow(unused_imports)]
use core::fmt::{self, Write};

#[allow(unused_imports)]
use aux11::{entry, iprint, iprintln, usart1};

macro_rules! uprint {
    ($serial:expr, $($arg:tt)*) => {
        $serial.write_fmt(format_args!($($arg)*)).ok()
    };
}

macro_rules! uprintln {
    ($serial:expr, $fmt:expr) => {
        uprint!($serial, concat!($fmt, "\n"))
    };
    ($serial:expr, $fmt:expr, $($arg:tt)*) => {
        uprint!($serial, concat!($fmt, "\n"), $($arg)*)
    };
}

#[allow(dead_code)]
struct SerialPort {
    usart1: &'static mut usart1::RegisterBlock,
}

impl fmt::Write for SerialPort {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        // Send a single character
        for c in s.chars() {
            while self.usart1.isr.read().txe().bit_is_clear() {} // <- NEW!
            self.usart1
            .tdr
            .write(|w| w.tdr().bits(u16::from(c as u8)) );
        }
        Ok(())
    }
}

#[entry]
fn main() -> ! {
    let (usart1, _mono_timer, mut _itm) = aux11::init();

    let mut serial = SerialPort {usart1};

    uprintln!(serial, "the answer is {}", 40 + 2);

    loop {}
}
