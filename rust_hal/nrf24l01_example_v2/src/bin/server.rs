#![deny(unsafe_code)]
#![no_std]
#![no_main]
extern crate panic_halt;

use cortex_m_semihosting::hprintln;
use cortex_m_rt::entry;
use cortex_m::asm::nop;
use embedded_hal::digital::v2::OutputPin;
use stm32f1xx_hal::{device, 
                    prelude::*,
                    spi::{Spi, Mode, Polarity, Phase},};
use embedded_nrf24l01::{NRF24L01, Configuration};

#[entry]
fn main() -> ! {
    // let core = cortex_m::Peripherals::take().unwrap();
    let device = device::Peripherals::take().unwrap();

    let mut flash = device.FLASH.constrain();
    let mut rcc = device.RCC.constrain();
    let mut afio = device.AFIO.constrain(&mut rcc.apb2);
    let mut gpioa = device.GPIOA.split(&mut rcc.apb2);
    let mut gpiob = device.GPIOB.split(&mut rcc.apb2);
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut led = gpiob.pb12.into_push_pull_output(&mut gpiob.crh);
    led.set_low().unwrap();

    let ce = gpiob.pb0.into_push_pull_output(&mut gpiob.crl);
    let csn = gpioa.pa4.into_push_pull_output(&mut gpioa.crl);
    // csn.set_high().unwrap();
    let sck = gpioa.pa5.into_alternate_push_pull(&mut gpioa.crl);
    let miso = gpioa.pa6;
    let mosi = gpioa.pa7.into_alternate_push_pull(&mut gpioa.crl);
    let spi = Spi::spi1(
        device.SPI1,
        (sck, miso, mosi),
        &mut afio.mapr,
        Mode {
            polarity: Polarity::IdleLow,
            phase: Phase::CaptureOnFirstTransition,
        },
        8.mhz(),
        clocks,
        &mut rcc.apb2);

    // nRF24L01 library specific starts here.
    let mut nrf24 = NRF24L01::new(ce, csn, spi).unwrap();
    nrf24.flush_rx().unwrap();
    let mut rx = nrf24.rx().unwrap();
    rx.set_pipes_rx_enable(&[true, true, false, false, false, false]).unwrap();
    rx.set_rx_addr(1, b"serv1").unwrap();
    rx.set_frequency(1).unwrap();
    led.set_high().unwrap();

    loop {
        if let Ok(false) =  rx.is_empty() {
            if let Ok(payload) = rx.read() {
                led.set_low().unwrap();
                // hprintln!("Payload: {:?}", payload.as_ref()).unwrap();
            }
        }
        for _ in 0..1_000000 { nop(); }
        /*
        if let Ok(pipe_option) = rx.can_read() {
            hprintln!("Pipe option: {:?}", pipe_option).unwrap();
            // if let Some(pipe_number) = pipe_option {
                if let Ok(payload) = rx.read() {
                    hprintln!("Payload: {:?}", payload.as_ref()).unwrap();
                    // if payload.len() > 0 {
                        // led.set_low().unwrap();
                        // hprintln!("Payload: {:?}", payload.as_ref()).unwrap();
                    // } else {
                        // led.set_high().unwrap();
                    // }
                }
            // }
            for _ in 0..1_000000 { nop(); }
        }
        */
    }
}
