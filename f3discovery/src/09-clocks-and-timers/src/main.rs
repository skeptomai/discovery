#![no_main]
#![no_std]

use aux9::{entry, switch_hal::OutputSwitch, tim6};

#[inline(never)]
fn delay(tim6: &tim6::RegisterBlock, ms: u16) {

    // Set auto-reload
    tim6.arr.write(|w| w.arr().bits(ms));

    // Enable the counter (CEN)
    tim6.cr1.modify(|_, w| w.opm().set_bit().cen().set_bit());

    // Wait for the counter to go off
    while !tim6.sr.read().uif().bit_is_set() {}

    // Clear the update event flag
    tim6.sr.modify(|_,w| w.uif().clear_bit());
}

#[entry]
fn main() -> ! {
    let (leds, rcc, tim6) = aux9::init();
    let mut leds = leds.into_array();

    // Power up the timer
    rcc.apb1enr.modify(|_,w| w.tim6en().set_bit());
    // Configure the timer to one-pulse mode
    tim6.cr1.write(|w| w.opm().set_bit().cen().clear_bit());
    // Configure the prescaler to have the counter operate at 1 KHz
    // APB1_CLOCK = 8 MHz
    // PSC = 7999
    // 8 MHz / (7999 + 1) = 1 KHz    
    tim6.psc.write(|w| w.psc().bits(7_999));

    let ms = 50;
    loop {
        for curr in 0..8 {
            let next = (curr + 1) % 8;

            leds[next].on().unwrap();
            delay(tim6, ms);
            leds[curr].off().unwrap();
            delay(tim6, ms);
        }
    }
}
