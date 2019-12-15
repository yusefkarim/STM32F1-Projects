#![deny(unsafe_code)]
#![no_std]
#![no_main]
extern crate panic_halt;

use lib::system_clock_init;
use core::cell::RefCell;
use core::ops::Deref;
use cortex_m::interrupt::{self, Mutex};
use cortex_m::{asm::wfi, peripheral::syst::SystClkSource};
use cortex_m_rt::{entry, exception};
use stm32f1::stm32f103;

static PC: Mutex<RefCell<Option<stm32f103::GPIOC>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    let core_peripherals = cortex_m::Peripherals::take().unwrap();
    let board_peripherals = stm32f103::Peripherals::take().unwrap();
    let mut systick = core_peripherals.SYST;
    let gpioc = board_peripherals.GPIOC;
    let flash = board_peripherals.FLASH;
    let rcc = board_peripherals.RCC;

    system_clock_init(&flash, &rcc);

    /* SysTick configuration */
    systick.set_clock_source(SystClkSource::Core);
    // Reload value must be less than 0x00FFFFFF
    systick.set_reload(1_440_000 - 1);
    systick.clear_current();

    /* GPIO Port C configuration */
    // Enable PC GPIO clock
    rcc.apb2enr.write(|w| w.iopcen().set_bit());
    // Set pin 13 as output, push/pull
    gpioc.crh.write(|w| w.mode13().output().cnf13().push_pull());
    gpioc.odr.write(|w| w.odr13().low());
    interrupt::free(|cs| {
        PC.borrow(cs).replace(Some(gpioc));
    });

    /* Enable SysTick counter and interrupt */
    systick.enable_counter();
    systick.enable_interrupt();

    loop {
        wfi();
    }
}

#[exception]
fn SysTick() {
    interrupt::free(|cs| {
        if let Some(ref gpioc) = PC.borrow(cs).borrow().deref() {
            if gpioc.odr.read().odr13().is_high() {
                gpioc.odr.write(|w| w.odr13().low());
            } else {
                gpioc.odr.write(|w| w.odr13().high());
            }
        }
    });
}
