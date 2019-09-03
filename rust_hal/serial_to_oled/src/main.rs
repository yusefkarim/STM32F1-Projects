#![deny(unsafe_code)]
#![no_std]
#![no_main]
extern crate panic_halt;

use nb::block;
use cortex_m_rt::{entry, exception, ExceptionFrame};
use stm32f1xx_hal::{
    prelude::*,
    pac,
    gpio::State,
    serial::{Config, StopBits, Serial},
    i2c::{BlockingI2c, DutyCycle, Mode},
};
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

    /* GPIO Port B USART3 configuration */
    let tx = gpiob.pb10.into_alternate_push_pull(&mut gpiob.crh);
    let rx = gpiob.pb11;
    let serial = Serial::usart3(
        board_peripherals.USART3,
        (tx, rx),
        &mut afio.mapr,
        Config::default()
            .baudrate(9600.bps())
            .stopbits(StopBits::STOP1)
            .parity_none(),
        clocks,
        &mut rcc.apb1,
    );
    let (mut tx, mut rx) = serial.split();

    /* GPIO Port C configuration */
    let mut gpioc = board_peripherals.GPIOC.split(&mut rcc.apb2);
    let mut onboard_led = gpioc.pc13.into_push_pull_output_with_state(
        &mut gpioc.crh,
        State::Low
    );

    /* ssd1306 driver setup */
    let mut disp: TerminalMode<_> = Builder::new().connect_i2c(i2c).into();
    // disp.set_rotation(DisplayRotation::Rotate180).unwrap();
    disp.init().unwrap();
    disp.clear().unwrap();

    let intro_message = b"Welcome! Enter messages to display:\n";
    for byte in intro_message.iter() {
        block!(tx.write(*byte)).ok();
    }

    let mut received: u8;
    loop {
        received = block!(rx.read()).unwrap();
        match received {
            b'~' => disp.clear().unwrap(),
            b'^' => onboard_led.toggle().unwrap(), 
            _ => disp.print_char(received as char).unwrap(),
        }
    }
}

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}
