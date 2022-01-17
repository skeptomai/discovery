#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux7::{entry, iprint, iprintln, ITM, RegisterBlock};
use switch_hal::OutputSwitch;

#[entry]
fn main() -> ! {
    
    let (_, gpioe, (mut step, mut direction)) = aux7::init();

    gpioe.bsrr.write(|w| w.bs9().set_bit());
    gpioe.bsrr.write(|w| w.bs11().set_bit());

    step.on().unwrap();
    direction.on().unwrap();


/*
    unsafe {
        // A magic address!
        const GPIOE_BSRR: u32 = 0x48001018;

        // Turn on the "North" LED (red)
        *(GPIOE_BSRR as *mut u32) = 1 << 9;

        // Turn on the "East" LED (green)
        *(GPIOE_BSRR as *mut u32) = 1 << 11;

        // Turn off the "North" LED
        *(GPIOE_BSRR as *mut u32) = 1 << (9 + 16);

        // Turn off the "East" LED
        *(GPIOE_BSRR as *mut u32) = 1 << (11 + 16);
    }
 */    
    loop {}
}
