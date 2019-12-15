#![no_std]
#![no_main]
#![deny(unsafe_code)]
extern crate panic_halt;

use lib::system_clock_init;
use cortex_m::asm::nop;
use cortex_m_rt::entry;
use stm32f1::stm32f103;

/// Address we are using as the Master
const MASTER_ADDR: u8 = 0x40;
/// The ADXL345 has an I2C address of 0x53
const ADXL345_ADDR: u8 = 0x53;

#[entry]
fn main() -> ! {
    // let core_peripherals = cortex_m::Peripherals::take().unwrap();
    let board_peripherals = stm32f103::Peripherals::take().unwrap();
    let rcc = board_peripherals.RCC;
    let flash = board_peripherals.FLASH;
    let gpiob = board_peripherals.GPIOB;
    let i2c1 = board_peripherals.I2C1;

    system_clock_init(&flash, &rcc);
    // Enable clocks
    rcc.apb1enr.write(|w| w.i2c1en().set_bit());
    rcc.apb2enr.write(|w| w.iopben().set_bit());
    // Reset I2C1
    // rcc.apb1rstr.write(|w| w.i2c1rst().reset());

    // Set PB6 (SCL1) and PB7 (SDA1) as open drain output
    gpiob.crl.write(|w| w.mode6().output50().cnf6().open_drain()
                         .mode7().output50().cnf7().open_drain());

    // Configure and start I2C1
    i2c1.cr1.write(|w| w.pe().disabled()
                        .smbus().i2c()
                        .nostretch().enabled());
    
    // TODO:

    loop {
        nop();
    }
}
