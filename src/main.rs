#![deny(unsafe_code)]
#![no_main]
#![no_std]

// Halt on panic
use panic_halt as _;

use cortex_m_rt::entry;
use stm32f4xx_hal as hal;
use hal::spi::Spi;
use hal::spi::{Mode, Phase, Polarity};
use crate::hal::{prelude::*, stm32};

#[entry]
fn main() -> ! {

    if let (Some(dp), Some(cp)) = (
        stm32::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        // Set up the system clock.
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.freeze();
        let mut delay = hal::delay::Delay::new(cp.SYST, clocks);
        let gpioa = dp.GPIOA.split();
        let (mut cs, clk, mosi) = (
            gpioa.pa4.into_push_pull_output(),
            gpioa.pa5.into_alternate_af5(),
            gpioa.pa7.into_alternate_af5(),
        );

        let gpiob = dp.GPIOB.split();
        let miso = gpiob.pb4.into_alternate_af5();

        let mut spi = Spi::spi1(
            dp.SPI1,
            (clk, miso, mosi),
            Mode {
                polarity: Polarity::IdleLow,
                phase: Phase::CaptureOnFirstTransition,
            },
            hal::time::KiloHertz(2000).into(),
            clocks
        );

        // CS HIGH = Standby mode
        cs.set_high();
        delay.delay_ms(100_u32);
        cs.set_low();

        let mfgId = spi.transfer(&mut [0x9F]);
        cs.set_high();
        loop {
            continue;
        }

    } else {
        loop {
            cortex_m::asm::nop();
        }
    }


}