#![deny(unsafe_code)]
#![no_std]
#![no_main]
extern crate panic_halt;

use cortex_m::asm::nop;
use cortex_m_rt::{entry, exception, ExceptionFrame};
use stm32f1xx_hal::{
    prelude::*,
    pac,
    i2c::{BlockingI2c, DutyCycle, Mode},
};
use core::fmt::Write;
use ssd1306::{prelude::*, Builder};

#[entry]
fn main() -> ! {
    let board_peripherals = pac::Peripherals::take().unwrap();

    // Take ownership of the raw FLASH and RCC devices, converting them
    // to HAL structs
    let mut flash = board_peripherals.FLASH.constrain();
    let mut rcc = board_peripherals.RCC.constrain();

    // Freeze the configuration of all clocks, storing them in clocks var
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    /* GPIO Port B I2C configuration */
    let mut afio = board_peripherals.AFIO.constrain(&mut rcc.apb2);
    let mut gpiob = board_peripherals.GPIOB.split(&mut rcc.apb2);
    let scl = gpiob.pb8.into_alternate_open_drain(&mut gpiob.crh);
    let sda = gpiob.pb9.into_alternate_open_drain(&mut gpiob.crh);
    let i2c = BlockingI2c::i2c1(
        board_peripherals.I2C1,
        (scl, sda),
        &mut afio.mapr,
        Mode::Fast {
            frequency: 400_000,
            duty_cycle: DutyCycle::Ratio2to1,
        },
        clocks,
        &mut rcc.apb1,
        1000,
        10,
        1000,
        1000,
    );

    /* GPIO Port C configuration */
    /* let mut gpioc = board_peripherals.GPIOC.split(&mut rcc.apb2);
    let mut onboard_led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
    */

    /* ssd1306 driver setup */
    let mut disp: TerminalMode<_> = Builder::new().connect_i2c(i2c).into();
    // disp.set_rotation(DisplayRotation::Rotate180).unwrap();
    disp.init().unwrap();
    disp.clear().unwrap();

    disp.write_str("Hello world!    ").unwrap();
    disp.write_str("Scooby Doo").unwrap();

    loop {
        nop();
    }
}

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}
