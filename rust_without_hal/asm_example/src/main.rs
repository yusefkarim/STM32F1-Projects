// #![deny(unsafe_code)]
#![no_std]
#![no_main]
#![feature(asm)]
extern crate panic_halt;

// use lib::system_clock_init;
use cortex_m::asm::nop;
use cortex_m_rt::entry;
use stm32f1::stm32f103;

#[entry]
fn main() -> ! {
    // let core_peripherals = cortex_m::Peripherals::take().unwrap();
    let board_peripherals = stm32f103::Peripherals::take().unwrap();
    let gpioc = board_peripherals.GPIOC;
    // let flash = board_peripherals.FLASH;
    let rcc = board_peripherals.RCC;

    // system_clock_init(&flash, &rcc);

    // Enable PC GPIO clock
    rcc.apb2enr.write(|w| w.iopcen().set_bit());
    // Set pin 13 as output, push/pull
    gpioc.crh.write(|w| w.mode13().output().cnf13().push_pull());
    gpioc.odr.write(|w| w.odr13().high());

    // unsafe {
        // asm!("movt  r3, #0x2000
              // mov   r3, #0x4000
              // movt  r2, #0x1234
              // mov   r2, #0x5678
              // str r2, [r3, #4]");
    // }
    let addr: u32 = 0x20003000;
    // let mut value: u32;
    unsafe {
        asm!("mov   r2, #0x1234
              str   r2, [$0]"
              :                                          // outputs
              :  "r"(addr)                               // inputs
              :  "r2"                                    // clobbers
              :                                          // options
        );
    }
    gpioc.odr.write(|w| w.odr12().low());
    // gpioc.odr.write(|w| w.odr13().low());

    loop {
        nop();
    }
}
