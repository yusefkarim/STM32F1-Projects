#![deny(unsafe_code)]
#![no_std]
#![no_main]
extern crate panic_halt;

use nb::block;
use cortex_m::{self, singleton};
use cortex_m_rt::entry;
use stm32f1xx_hal::{
    prelude::*,
    pac,
    serial::{Config, StopBits, Serial},
};


#[entry]
fn main() -> ! {
    let board_peripherals = pac::Peripherals::take().unwrap();

    // Take ownership of the raw FLASH and RCC devices, converting them
    // to HAL structs
    let mut flash = board_peripherals.FLASH.constrain();
    let mut rcc = board_peripherals.RCC.constrain();
    // Alternate GPIO register
    let mut afio = board_peripherals.AFIO.constrain(&mut rcc.apb2);
    // Channels for DMA1 controller
    let channels = board_peripherals.DMA1.split(&mut rcc.ahb);

    // Freeze the configuration of all clocks, storing them in clocks var
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    /* GPIO Port B USART3 configuration */
    let mut gpiob = board_peripherals.GPIOB.split(&mut rcc.apb2);
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

    let (mut tx, rx) = serial.split();
    let mut rx_option = Some(rx.with_dma(channels.3));
    let mut buf_option = Some(singleton!(: [u8; 32] = [0; 32]).unwrap());

    let intro_message = b"Welcome! Type some stuff: ";
    for byte in intro_message.iter() {
        block!(tx.write(*byte)).ok();
    }

    let mut curr_len: usize = 0;
    loop {
        let rx = rx_option.take().unwrap();
        let buf = buf_option.take().unwrap();
        let transfer = rx.read(buf);
        while !transfer.is_done() {
            let transfer_slice = transfer.peek();
            if transfer_slice.len() != curr_len {
                if let Some(data) = transfer_slice.last() {
                    block!(tx.write(*data)).ok();
                }
                curr_len = transfer_slice.len();
            }
        }

        let (buf, rx) = transfer.wait();
        rx_option.replace(rx);
        buf_option.replace(buf);
    }
}
