#![deny(unsafe_code)]
#![no_std]
#![no_main]
extern crate panic_halt;

use cortex_m::{self, asm::nop};
use cortex_m_rt::entry;
use stm32f1::stm32f103;


#[entry]
fn main() -> ! {
    let core_peripherals = cortex_m::Peripherals::take().unwrap();
    let board_peripherals = stm32f103::Peripherals::take().unwrap();
    // let rcc = &board_peripherals.RCC;
    let mut scb = core_peripherals.SCB;
    let usb = board_peripherals.USB;

    usb.cntr.write(|w| w.fres().reset());

    for _ in 0..10_0000 {
        nop();
    }
    scb.system_reset();

    // loop {
        // nop();
    // }
}

