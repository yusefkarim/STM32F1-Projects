#![deny(unsafe_code)]
#![no_std]
#![no_main]
extern crate panic_halt;

use cortex_m_rt::entry;
use cortex_m::asm::nop;
use embedded_hal::digital::v2::OutputPin;
use stm32f1xx_hal::{device, prelude::*, spi::Spi};
use nrf24l01::NRF24L01;

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
    led.set_high().unwrap();

    let ce = gpiob.pb0.into_push_pull_output(&mut gpiob.crl);
    let mut csn = gpioa.pa4.into_push_pull_output(&mut gpioa.crl);
    csn.set_high().unwrap();
    let sck = gpioa.pa5.into_alternate_push_pull(&mut gpioa.crl);
    let miso = gpioa.pa6;
    let mosi = gpioa.pa7.into_alternate_push_pull(&mut gpioa.crl);
    let spi = Spi::spi1(
        device.SPI1,
        (sck, miso, mosi),
        &mut afio.mapr,
        nrf24l01::MODE,
        1.mhz(),
        clocks,
        &mut rcc.apb2,);

    // nRF24L01 library specific starts here.
    let mut nrf24l01 = NRF24L01::new(spi, csn, ce, 1, 4).unwrap();
    nrf24l01.set_raddr("client1".as_bytes()).unwrap();
    nrf24l01.config().unwrap();
    led.set_low().unwrap();

    let mut buffer = [0; 4];
    loop {
        // if !nrf24l01.is_sending().unwrap() {
            // led.set_high().unwrap();
            // for _ in 0..1000000 { nop(); }
            // nrf24l01.set_taddr("serv1".as_bytes()).unwrap();
            // nrf24l01.send(&buffer).unwrap();
            // led.set_low().unwrap();
            // while !nrf24l01.data_ready().unwrap() { nop(); }
            // nrf24l01.get_data(&mut buffer).unwrap();
        // }
        if !nrf24l01.is_sending().unwrap() {
            for _ in 0..1000000 { nop(); }
            led.set_high().unwrap();
            nrf24l01.set_taddr("server1".as_bytes()).unwrap();
            nrf24l01.send(&buffer).unwrap();
            for _ in 0..1000000 { nop(); }
            led.set_low().unwrap();
        }
    }
}
