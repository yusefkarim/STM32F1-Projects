#![deny(unsafe_code)]
#![no_std]
#![no_main]
extern crate panic_halt;

// Rust core and external imports
use core::{str, fmt::Write};
use rtfm::app;
use rtfm::Peripherals;
use cortex_m::asm::delay;
use embedded_hal::digital::v2::OutputPin;
use stm32f1xx_hal::{
    prelude::*,
    delay::Delay,
    device::TIM1,
};
// use core::convert::TryInto;

// use cortex_m_semihosting::hprint;

#[app(device = stm32f1xx_hal::device)]
const APP: () = {
    static mut CORE: Peripherals<'static> = ();

    #[init]
    fn init() -> init::LateResources {
        let mut flash = device.FLASH.constrain();
        let mut rcc = device.RCC.constrain();
        // let scb = core.SCB;

        let clocks = rcc
            .cfgr
            .use_hse(8.mhz())
            .sysclk(72.mhz())
            .pclk1(36.mhz())
            .freeze(&mut flash.acr);

        // delay(clocks.sysclk().0);
        // scb.system_reset();
        // CORE = core.try_into().unwrap();
        init::LateResources {
            CORE: core,
        }
    }


    /*
    #[idle]
    fn idle() -> ! {
        loop {
            wfi();
        }
    }*/
};
