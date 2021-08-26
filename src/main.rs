#![deny(unsafe_code)]
#![no_main]
#![no_std]


// Halt on panic
use panic_halt as _;
// extern crate stsafe_rs;

use cortex_m_rt::entry;
use stm32f4xx_hal as hal;

use hal::{prelude::*, stm32};
use hal::{i2c::I2c, time::KiloHertz};

#[entry]
fn main() -> ! {

    if let (Some(dp), Some(cp)) = (
        stm32::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {

        // Set up the system clock.
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.use_hse(25.mhz()).freeze();
        let mut delay = hal::delay::Delay::new(cp.SYST, clocks);
        let gpiob = dp.GPIOB.split();        let mut trigger = gpiob.pb10.into_push_pull_output();

        trigger.set_low().ok();
        delay.delay_ms(250_u16);
        trigger.set_high().ok();
        delay.delay_ms(250_u16);
        trigger.set_low().ok();
        loop {
            continue;
        }

    } else {
        loop {
            cortex_m::asm::nop();
        }
    }


}