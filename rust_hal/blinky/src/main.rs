#![deny(unsafe_code)]
#![no_std]
#![no_main]
extern crate panic_halt;

use cortex_m_rt::entry;
use nb::block;
use embedded_hal::digital::v2::OutputPin;
use stm32f1xx_hal::{pac, prelude::*, timer::Timer};

#[entry]
fn main() -> ! {
    let core_peripherals = cortex_m::Peripherals::take().unwrap();
    let board_peripherals = pac::Peripherals::take().unwrap();

    // Take ownership of the raw FLASH and RCC devices, converting them
    // to HAL structs
    let mut flash = board_peripherals.FLASH.constrain();
    let mut rcc = board_peripherals.RCC.constrain();

    // Freeze the configuration of all clocks, storing them in clocks var
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    /* GPIO Port C configuration */
    let mut gpioc = board_peripherals.GPIOC.split(&mut rcc.apb2);
    let mut onboard_led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

    /* Enable SysTick counter to trigger an update every second */
    let mut systick = Timer::syst(core_peripherals.SYST, 10.hz(), clocks);

    loop {
        block!(systick.wait()).unwrap();
        onboard_led.toggle().unwrap();
    }
}
