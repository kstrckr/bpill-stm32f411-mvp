#![deny(unsafe_code)]
#![no_main]
#![no_std]

// Halt on panic
use panic_halt as _;

use cortex_m_rt::entry;
use stm32f4xx_hal as hal;
use hal::spi::Spi;
use hal::spi::{Mode, Phase, Polarity};
use hal::rcc::{Clocks, Rcc};
use crate::hal::{prelude::*, stm32};

fn setup_clocks(rcc: Rcc) -> Clocks {
    return rcc.cfgr.hclk(84.mhz()).sysclk(84.mhz()).pclk1(36.mhz()).pclk2(36.mhz()).freeze();
}
#[entry]
fn main() -> ! {

    if let (Some(dp), Some(cp)) = (
        stm32::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        // Set up the system clock.
        let rcc = dp.RCC.constrain();
        let clocks = setup_clocks(rcc);
        let mut delay = hal::delay::Delay::new(cp.SYST, clocks);
        let gpioa = dp.GPIOA.split();
        let (mut cs, clk, miso, mosi) = (
            gpioa.pa4.into_push_pull_output(),
            gpioa.pa5.into_alternate_af5(),
            gpioa.pa6.into_alternate_af5(),
            gpioa.pa7.into_alternate_af5(),
        );


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
        cs.set_high().unwrap();
        delay.delay_ms(100_u32);
        cs.set_low().unwrap();


        // let ready = spi.is_txe();
        spi.write(&[0x7C]);

        cs.set_high().unwrap();
        delay.delay_ms(100_u32);
        cs.set_low().unwrap();

        let cmd = &mut [0x9F, 0, 0, 0, 0, 0];
        let mfgId = spi.transfer(cmd).ok();
        let read = spi.read();
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