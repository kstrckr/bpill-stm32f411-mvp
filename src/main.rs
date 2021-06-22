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

        let gpioa = dp.GPIOA.split();
        let channels = (
            gpioa.pa8.into_alternate_af1(),
            gpioa.pa9.into_alternate_af1(),
        );

        let pwm = pwm::tim1(dp.TIM1, channels, clocks, 20u32.khz());
        let (mut ch1, _ch2) = pwm;
        let max_duty = ch1.get_max_duty();
        let mut current_duty = 800;
        let mut increasing = false;
        let mut delay = hal::delay::Delay::new(cp.SYST, clocks);
        ch1.enable();

        loop {
            if increasing {
                current_duty += 1;
            } else {
                current_duty -= 1;
            }
            if current_duty == 0 || current_duty == 800 {
                increasing = !increasing;
            }
            ch1.set_duty(max_duty - current_duty);
            delay.delay_us(750_u32);
        }

    } else {
        loop {
            cortex_m::asm::nop();
        }
    }


}