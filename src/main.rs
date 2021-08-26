#![deny(unsafe_code)]
#![no_main]
#![no_std]


// Halt on panic
use panic_halt as _;
extern crate stsafe_rs;

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
        let gpiob = dp.GPIOB.split();
        let (scl, sda) = (
            gpiob.pb6.into_alternate_af4_open_drain(),
            gpiob.pb7.into_alternate_af4_open_drain(),
        );

        let i2c = I2c::new(dp.I2C1, (scl, sda), KiloHertz(100), clocks);

        let mut stsafe = stsafe_rs::StsafeA110::new(i2c);
        let res_ok: bool = stsafe.reset(&mut delay);

        delay.delay_ms(35_u8);

        let mut buff: [u8; 8] = [0; 8];
        let random_bytes = stsafe.get_random_bytes(&mut delay, &mut buff[..]);

        loop {
            continue;
        }

    } else {
        loop {
            cortex_m::asm::nop();
        }
    }


}