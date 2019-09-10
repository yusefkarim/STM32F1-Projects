#![deny(unsafe_code)]
#![no_std]
#![no_main]
extern crate panic_halt;

// use cortex_m_semihosting::hprint;
use core::str;
use cortex_m::asm::delay;
use cortex_m_rt::entry;
// use embedded_hal::digital::v2::OutputPin;
// use stm32f1xx_hal::{prelude::*, pac};
use stm32f1xx_hal::{prelude::*, stm32};
use stm32_usbd::UsbBus;
use usbd_serial::{SerialPort, USB_CLASS_CDC};
use usb_device::prelude::{UsbDeviceBuilder, UsbVidPid};


#[entry]
fn main() -> ! {
    let board_peripherals = stm32::Peripherals::take().unwrap();

    let mut flash = board_peripherals.FLASH.constrain();
    let mut rcc = board_peripherals.RCC.constrain();
    let mut gpioc = board_peripherals.GPIOC.split(&mut rcc.apb2);
	let mut gpioa = board_peripherals.GPIOA.split(&mut rcc.apb2);
	
	let clocks = rcc.cfgr
                    .use_hse(8.mhz())
                    .sysclk(72.mhz())
                    .pclk1(24.mhz())
                    .freeze(&mut flash.acr);
    
	let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
	let mut usb_data_plus = gpioa.pa12.into_push_pull_output(&mut gpioa.crh);

	// Turn off on-board LED
    led.set_high();
	// Pull the D+ pin down to send a RESET condition to the USB bus
	usb_data_plus.set_low();
    // Delay for 720000 clock cycles
	delay(clocks.sysclk().0 / 100);
    
    let usb_data_minus = gpioa.pa11;
    let usb_data_plus = usb_data_plus.into_floating_input(&mut gpioa.crh);
    let usb_bus = UsbBus::new(board_peripherals.USB,
                              (usb_data_minus, usb_data_plus));
    let mut usb_serial = SerialPort::new(&usb_bus);
    let mut usb_device = UsbDeviceBuilder::new(&usb_bus,
        UsbVidPid(0x1337, 0x1337))
        .manufacturer("Completely Managed")
        .product("Sensor Board")
        .serial_number("0001")
        .device_class(USB_CLASS_CDC)
        .build();



    let mut buf = [0u8; 64];
    loop {
        if !usb_device.poll(&mut [&mut usb_serial]) {
            continue;
        }

        if let Ok(count) =  usb_serial.read(&mut buf) {
            if count > 0 {
                if let Ok(data) = str::from_utf8(&buf[0..count]) {
                    if data.contains("toggle") {
                        led.toggle();
                    }
                }
                // if buf[0..count].contains(b"toggle") {
                    // led.toggle();
                // }
                // serial.write(&buf[0..count]).ok();
                // for c in buf[0..count].iter() {
                // }
            }
        }
    }
}
