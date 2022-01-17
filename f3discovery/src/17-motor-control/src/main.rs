#![no_main]
#![no_std]

use aux17::_embedded_hal_digital_OutputPin;
use aux17::_embedded_hal_digital_ToggleableOutputPin;

#[allow(unused_imports)]

use aux17::{entry, tim6};

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

    let (_, (mut step, mut direction), rcc, tim6) = aux17::init();

    // Power up the timer
    rcc.apb1enr.modify(|_,w| w.tim6en().set_bit());
    // Configure the timer to one-pulse mode
    tim6.cr1.write(|w| w.opm().set_bit().cen().clear_bit());
    // Configure the prescaler to have the counter operate at 1 KHz
    // APB1_CLOCK = 8 MHz
    // PSC = 7999
    // 8 MHz / (7999 + 1) = 1 KHz    
    tim6.psc.write(|w| w.psc().bits(7_999));    

    // motor direction forward / right
    direction.set_high().unwrap();

    for _ in 0..2 {

        for _ in 0..400 {
            step.toggle().unwrap();
            delay(tim6, 10);
        }

        delay(tim6, 100);
        direction.toggle().unwrap();
        
    }


    loop {}
}