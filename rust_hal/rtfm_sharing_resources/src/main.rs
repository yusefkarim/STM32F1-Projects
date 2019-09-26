#![deny(unsafe_code)]
#![no_std]
#![no_main]
extern crate panic_halt;

// Rust core and external imports
use rtfm::app;
// use rtfm::Peripherals;
use stm32f1xx_hal::prelude::*;
// use embedded_hal::digital::v2::OutputPin;
// use core::convert::TryInto;

// use cortex_m_semihosting::hprint;

#[app(device = stm32f1xx_hal::device)]
const APP: () = {
    static mut CORE: cortex_m::Peripherals = ();

    #[init]
    fn init() -> init::LateResources {
        let mut flash = device.FLASH.constrain();
        let mut rcc = device.RCC.constrain();
        let core_peripherals = cortex_m::Peripherals::take().unwrap();
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
            CORE: core_peripherals,
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
