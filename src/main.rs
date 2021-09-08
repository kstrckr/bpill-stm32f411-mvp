// #![deny(unsafe_code)]
#![no_main]
#![no_std]


// Halt on panic
use panic_halt as _;
// extern crate stsafe_rs;

use cortex_m_rt::entry;
use cortex_m::interrupt::{free, CriticalSection, Mutex};

use stm32f4xx_hal as hal;
use hal::{prelude::*, stm32, syscfg::SysCfg};
use hal::{i2c::I2c, time::KiloHertz, interrupt, gpio::Edge, gpio::PullUp, gpio::Input, gpio::gpioa::PA0};
use core::cell::{Cell, RefCell};
use core::ops::DerefMut;

static BUTTON: Mutex<RefCell<Option<PA0<Input<PullUp>>>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {

    if let (Some(mut dp), Some(cp)) = (
        stm32::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {

        // Set up the system clock.
        let rcc = dp.RCC.constrain();

        let clocks = rcc
            .cfgr
            .sysclk(100.mhz())
            .hclk(48.mhz())
            .pclk1(24.mhz())
            .pclk2(24.mhz())
            .freeze();

        let mut delay = hal::delay::Delay::new(cp.SYST, clocks);

        let gpioa = dp.GPIOA.split();
        let mut syscfg = dp.SYSCFG.constrain();
        let mut btn = gpioa.pa0.into_pull_up_input();
        btn.make_interrupt_source(&mut syscfg);
        btn.enable_interrupt(&mut dp.EXTI);
        btn.trigger_on_edge(&mut dp.EXTI, Edge::FALLING);

        free(|cs| {
            BUTTON.borrow(cs).replace(Some(btn));
        });
        hal::pac::NVIC::unpend(hal::pac::Interrupt::EXTI0);
        unsafe {
            hal::pac::NVIC::unmask(hal::pac::Interrupt::EXTI0);
        };

        // let gpiob = dp.GPIOB.split();
        // let mut trigger = gpiob.pb10.into_push_pull_output();

        // let sustain = 100000_u32;
        // trigger.set_low().ok();
        // trigger.set_high().ok();
        // delay.delay_us(sustain);
        // trigger.set_low().ok();

        loop {
            continue;
        }

    } else {
        loop {
            cortex_m::asm::nop();
        }
    }
}


#[interrupt]
fn EXTI0() {
    free(|cs| {
        let mut btn_ref = BUTTON.borrow(cs).borrow_mut();
        if let Some(ref mut btn) = btn_ref.take() {
            btn.clear_interrupt_pending_bit();
        }
    });
}