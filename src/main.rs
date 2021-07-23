#![deny(unsafe_code)]
#![no_main]
#![no_std]

// Halt on panic
use panic_halt as _;

use cortex_m_rt::entry;
use stm32f4xx_hal as hal;
use crate::hal::{prelude::*, pwm, stm32};

#[entry]
fn main() -> ! {

    if let (Some(dp), Some(cp)) = (
        stm32::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        // Set up the system clock.
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.freeze();

        let gpiob = dp.GPIOB.split();

        let mut segSel0 = gpiob.pb6.into_push_pull_output();
        segSel0.set_high();

        let mut charSel3 = gpiob.pb12.into_push_pull_output();
        charSel3.set_low();

        loop {
            continue;
        }

    } else {
        loop {
            cortex_m::asm::nop();
        }
    }


}