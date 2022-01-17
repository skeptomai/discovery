#![deny(unsafe_code)]
#![no_main]
#![no_std]

use volatile::Volatile;
use aux5::{entry, Delay, DelayMs, LedArray, OutputSwitch};

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, LedArray) = aux5::init();

    let mut half_period = 500_u16;
    let v_half_period = Volatile::new(&mut half_period);

/*
    // fully parametrized on i (on time)
    let mut i = 0;
    loop {
        if i % 2 != 0 {
            let on = ((i+1)/2) % 8;
            let off = (((i+1)/2)-1) % 8;
            leds[on].on().ok();
            leds[off].off().ok();
            delay.delay_ms(v_half_period.read());
        } else {
            let on1 = (i/2) % 8;
            let on2 = ((i/2)+1) % 8;
            leds[on1].on().ok();
            leds[on2].on().ok();
            delay.delay_ms(v_half_period.read());
        }
        i = (i+1) % 16;
    }
 */
/*
// my shorter solution. also more 'correct' than author's
// in that it starts with first led at time 0
    loop {
        for curr in 0..8 {
            leds[curr].on().ok();
            delay.delay_ms(v_half_period.read());
            let next = (curr+1) % 8;
            leds[next].on().ok();
            delay.delay_ms(v_half_period.read());
            leds[curr].off().ok();
        }
    }
 */    

    // tutorial author's solution
    loop {
        for curr in 0..8 {
            let next = (curr + 1) % 8;

            leds[next].on().ok();
            delay.delay_ms(v_half_period.read());
            leds[curr].off().ok();
            delay.delay_ms(v_half_period.read());
        }
    }
}
