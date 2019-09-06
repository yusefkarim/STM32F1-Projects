#![deny(unsafe_code)]
#![no_std]
#![no_main]
extern crate panic_halt;

use cortex_m_semihosting::hprint;
use cortex_m::asm::nop;
use cortex_m_rt::{entry};
use stm32f1::stm32f103;


#[entry]
fn main() -> ! {
    // let core_peripherals = cortex_m::Peripherals::take().unwrap();
    let board_peripherals = stm32f103::Peripherals::take().unwrap();
    let rcc = board_peripherals.RCC;
    let gpiob = board_peripherals.GPIOB;
    // let afio = board_peripherals.AFIO;
    let usart3 = board_peripherals.USART3;

    // Enable clock for GPIOB (PB) and USART3
    rcc.apb2enr.write(|w| w.iopben().enabled());
    rcc.apb1enr.write(|w| w.usart3en().enabled());

    /* PB10 = USART3_TX, PB11 = USART3_RX */
    // PB10 (TX) as alternate function push/pull output
    gpiob.crh.write(|w| w.mode10().output50().cnf10().alt_push_pull());
    // PB11 as pull-up/pull-down input
    gpiob.crh.write(|w| w.mode11().input().cnf11().alt_push_pull());
    // Set PB11 as pull-up
    gpiob.odr.write(|w| w.odr11().set_bit());
    // Make sure default mappings are used (maps to PB10-11)
    // unsafe { afio.mapr.write(|w| w.usart3_remap().bits(0)) };

    /* Setting up USART3 RX and TX */
    // Disable USART, set data length to 8 bits, with no parity
    usart3.cr1.write(|w| w.ue().disabled()
                          .m().m8()
                          .pce().disabled());
    // 1 stop bit
    usart3.cr2.write(|w| w.stop().stop1());
    // Baudrate of 9600 assuming 36 MHz clock (see Section 27.3.4 of manual)
    // usart3.brr.write(|w| w.div_mantissa().bits(0xEA).div_fraction().bits(0x6));
    // Baudrate of 9600 assuming 8 MHz clock (see Section 27.3.4 of manual)
    usart3.brr.write(|w| w.div_mantissa().bits(0x34).div_fraction().bits(0x1));
    // Enable transmission and reception, then enable USART
    usart3.cr1.write(|w| w.ue().enabled().re().enabled().te().enabled());

    hprint!("Starting to read serial data...\n").unwrap();
    let mut read_byte: u16;
    loop {
        while !usart3.sr.read().rxne().bit_is_set() { nop(); }
        read_byte = usart3.dr.read().dr().bits();
        while !usart3.sr.read().txe().bit_is_set() { nop(); }
        usart3.dr.write(|w| w.dr().bits(read_byte));
        while !usart3.sr.read().tc().bit_is_set() { nop(); }
        usart3.sr.write(|w| w.tc().clear_bit());
    }
}
