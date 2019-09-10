#![deny(unsafe_code)]
#![no_std]
#![no_main]
extern crate panic_halt;

use nb::block;
use cortex_m::{self, asm::nop};
// use embedded_hal::digital::v2::OutputPin;
use cortex_m_rt::entry;
use stm32f1::stm32f103::{interrupt, Interrupt};
use stm32f1xx_hal::{
    prelude::*,
    pac,
	delay,
    serial::{Config, StopBits, Serial},
};
// use core::fmt::Write;
use hd44780_driver::{Cursor, CursorBlink, Display, DisplayMode, HD44780};
use cortex_m_semihosting::hprint;

/*
macro_rules! read_and_write_line {
	($rx: expr, $lcd: expr) => {
        let mut ch: char;
        loop {
            match block!($rx.read()) {
                Ok(byte) => ch = byte as char,
                Err(err) => {
                    hprintln!("{:?}", err).unwrap();
                    ch = 'x';
                }
            };
            if ch == '*' {
                break;
            }
            else {
                $lcd.write_char(ch);
            }
        }
    };
}*/

#[entry]
fn main() -> ! {
    let core_peripherals = cortex_m::Peripherals::take().unwrap();
    let board_peripherals = pac::Peripherals::take().unwrap();

    // Take ownership of the raw FLASH and RCC devices, converting them
    // to HAL structs
    let mut flash = board_peripherals.FLASH.constrain();
    let mut rcc = board_peripherals.RCC.constrain();
    // Alternate GPIO register
    let mut afio = board_peripherals.AFIO.constrain(&mut rcc.apb2);
    let mut nvic = core_peripherals.NVIC;

    // Freeze the configuration of all clocks, storing them in clocks var
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let delay = delay::Delay::new(core_peripherals.SYST, clocks);

    /* GPIO configuration for HD44780 LCD  */
    // let mut gpioa = board_peripherals.GPIOA.split(&mut rcc.apb2);
    let mut gpiob = board_peripherals.GPIOB.split(&mut rcc.apb2);
	let d4 = gpiob.pb12.into_push_pull_output(&mut gpiob.crh);
	let d5 = gpiob.pb13.into_push_pull_output(&mut gpiob.crh);
    let d6 = gpiob.pb14.into_push_pull_output(&mut gpiob.crh);
    let d7 = gpiob.pb15.into_push_pull_output(&mut gpiob.crh);
    let rs = gpiob.pb8.into_push_pull_output(&mut gpiob.crh);
    let en = gpiob.pb7.into_push_pull_output(&mut gpiob.crl);
    let mut lcd = HD44780::new_4bit(rs, en, d4, d5, d6, d7, delay);

    /* GPIO Port B USART3 configuration */
    let tx = gpiob.pb10.into_alternate_push_pull(&mut gpiob.crh);
    let rx = gpiob.pb11;
    let serial = Serial::usart3(
        board_peripherals.USART3,
        (tx, rx),
        &mut afio.mapr,
        Config::default()
            .baudrate(9600_u32.bps())
            .stopbits(StopBits::STOP1)
            .parity_none(),
        clocks,
        &mut rcc.apb1,
    );

    let (mut tx, mut rx) = serial.split();

    lcd.set_display_mode(
        DisplayMode {
            display: Display::On,
            cursor_visibility: Cursor::Invisible,
            cursor_blink: CursorBlink::Off,
        }
    );

    rx.listen();
    nvic.enable(Interrupt::USART3);

    lcd.reset();
    lcd.clear();


    // lcd.write_str(top_msg).unwrap();
    let intro_message = b"Welcome! Enter messages to display: ";
    for byte in intro_message.iter() {
        block!(tx.write(*byte)).ok();
    }

    // let mut trigger_byte: u8;
    loop {
        nop();
        // hprintln!("{:?}", buf).unwrap();
        // rx_option.replace(rx);
        // buf_option.replace(buf);
        // if _buf.len() > zero {
        // for x in _buf.as_mut() {
            // hprintln!("{}", x).unwrap();
        // }
        // let (_buf, _rx) = rx.read(buf).wait();
        /*
        trigger_byte = block!(rx.read()).unwrap();
        match trigger_byte {
            b'~' => {
                lcd.clear();
                read_and_write_line!(rx, lcd);
                lcd.set_cursor_pos(40);
                read_and_write_line!(rx, lcd);
            }
            b'^' => (),
            _ => (),
        }*/
    }
}


#[interrupt]
fn USART3() {
    hprint!("Oops").unwrap();
}
// ~Hello*world!*
