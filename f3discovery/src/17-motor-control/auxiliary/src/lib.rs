//! Initialization code

#![deny(warnings)]
#![no_std]

use panic_itm as _; // panic handler

pub use cortex_m::{asm::bkpt, iprint, iprintln, peripheral::ITM, peripheral::SYST};
pub use cortex_m_rt::entry;

pub use stm32f3::stm32f303::{self, gpioc::RegisterBlock};
pub use stm32f3_discovery::stm32f3xx_hal::pac::GPIOA;
pub use stm32f3_discovery::switch_hal::IntoSwitch;
pub use stm32f3_discovery::{stm32f3xx_hal};
pub use stm32f3::stm32f303::{rcc, tim6, RCC, TIM6};
pub use stm32f3xx_hal::{stm32, gpio::{Output, PushPull, gpioa::PA2, gpioa::PA3},prelude::*, pac, timer::Timer};
pub use switch_hal::{ActiveHigh, Switch};

#[inline(never)]
pub fn init() -> (ITM, (PA2<Output<PushPull>>, PA3<Output<PushPull>>),
    &'static rcc::RegisterBlock,
    &'static tim6::RegisterBlock,) {
    let device_periphs = stm32::Peripherals::take().unwrap();
    //let _cp = cortex_m::Peripherals::take().unwrap();
    //let _pac = pac::Peripherals::take().unwrap();
    let mut reset_and_clock_control = device_periphs.RCC.constrain();

    let mut _gpioa = device_periphs.GPIOA.split(&mut reset_and_clock_control.ahb);
    let motor_control = (
                _gpioa.pa2
                    .into_push_pull_output(&mut _gpioa.moder, &mut _gpioa.otyper),
                _gpioa.pa3
                    .into_push_pull_output(&mut _gpioa.moder, &mut _gpioa.otyper));


    let core_periphs = cortex_m::Peripherals::take().unwrap();
    (core_periphs.ITM, motor_control, unsafe { &*RCC::ptr() }, unsafe { &*TIM6::ptr() })
}
