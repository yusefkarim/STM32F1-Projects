#![deny(unsafe_code)]
#![no_std]
#![no_main]
extern crate panic_halt;

// use core::sync::atomic::{AtomicBool, Ordering};
use core::ops::Deref;
use core::cell::RefCell;
use cortex_m::{self, peripheral::syst::SystClkSource, asm::nop};
use cortex_m::interrupt::{self, Mutex};
use cortex_m_rt::{entry, exception};
use stm32f1::stm32f103;

// static SYSTICK_EVENT: AtomicBool = AtomicBool::new(false);
static PC: Mutex<RefCell<Option<stm32f103::GPIOC>>> =
             Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    let core_peripherals = cortex_m::Peripherals::take().unwrap();
    let board_peripherals = stm32f103::Peripherals::take().unwrap();
    let mut systick = core_peripherals.SYST;
    let gpioc = &board_peripherals.GPIOC;
    let rcc = &board_peripherals.RCC;

    /* SysTick configuration */
    systick.set_clock_source(SystClkSource::Core);
    systick.set_reload(72_000_000);
    systick.clear_current();

    /* GPIO Port C configuration */
    // Enable PC GPIO clock
    rcc.apb2enr.write(|w| w.iopcen().set_bit());
    // Set pin 13 as output, push/pull
    gpioc.crh.write(|w| w.mode13().output().cnf13().push_pull());
    gpioc.odr.write(|w| w.odr13().low());
    interrupt::free(|cs| {
        PC.borrow(cs).replace(Some(board_peripherals.GPIOC));
    });

    /* Enable SysTick counter and interrupt */
    systick.enable_counter();
    systick.enable_interrupt();

    loop {
        nop();
        // if SYSTICK_EVENT.compare_and_swap(true, false, Ordering::Relaxed) {
            // if gpioc.odr.read().odr13().is_high() {
                // gpioc.odr.write(|w| w.odr13().low());
            // } else {
                // gpioc.odr.write(|w| w.odr13().high());
            // }
        // }
    }
}

#[exception]
fn SysTick() {
    // SYSTICK_EVENT.compare_and_swap(false, true, Ordering::Relaxed);
    interrupt::free(|cs| {
        if let Some(ref gpioc) = PC.borrow(cs).borrow().deref() {
            gpioc.odr.write(|w| w.odr13().high());
        }
        // if gpioc.as_ref().unwrap().odr.read().odr13().is_high() {
            // gpioc.as_ref().unwrap().odr.write(|w| w.odr13().low());
        // } else {
            // gpioc.as_ref().unwrap().odr.write(|w| w.odr13().high());
        // }
    });
}
