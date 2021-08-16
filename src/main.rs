#![deny(unsafe_code)]
#![no_main]
#![no_std]


use embedded_hal::blocking::serial::write;
// Halt on panic
use panic_halt as _;
extern crate stsafe_rs;

use cortex_m_rt::entry;
use stm32f4xx_hal as hal;

use hal::{prelude::*, stm32};
use hal::{i2c::I2c, time::KiloHertz};

use crc::{Crc, Algorithm, CRC_16_IBM_SDLC};
// fn generateRandomCommand() -> [u8; 1] {
//     let size = 32_u8;
    
//     // cmd header mask = 
//     // 0b1110000
    
//     let i: [u8; 1] = [0];
//     i

// polling max 2500ms
// polling step 3ms
// }


#[entry]
fn main() -> ! {
    pub const X25: Crc<u16> = Crc::<u16>::new(&CRC_16_IBM_SDLC);

    if let (Some(dp), Some(cp)) = (
        stm32::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {

        // Set up the system clock.
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.use_hse(8.mhz()).freeze();
        let mut delay = hal::delay::Delay::new(cp.SYST, clocks);
        let gpiob = dp.GPIOB.split();
        let (scl, sda) = (
            gpiob.pb6.into_alternate_af4_open_drain(),
            gpiob.pb7.into_alternate_af4_open_drain(),
        );

        let mut i2c = I2c::i2c1(dp.I2C1, (scl, sda), KiloHertz(100), clocks);

        let mut stsafe = stsafe_rs::StsafeA110::new(i2c);
        let okay: bool = stsafe.reset(&mut delay);

        delay.delay_ms(35_u8);

        let randomBytes = stsafe.getRandomBytes(&mut delay, 10);

        loop {
            continue;
        }

    } else {
        loop {
            cortex_m::asm::nop();
        }
    }


}