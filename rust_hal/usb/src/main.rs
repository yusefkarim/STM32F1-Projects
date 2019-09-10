#![deny(unsafe_code)]
#![no_std]
#![no_main]
extern crate panic_halt;

use rtfm::app;
use core::str;
use cortex_m::asm::delay;
// use stm32f1xx_hal::{prelude::*, pac};
// use embedded_hal::digital::v2::OutputPin;
use stm32f1xx_hal::prelude::*;
use stm32_usbd::{UsbBusType, UsbBus};
use usbd_serial::{SerialPort, USB_CLASS_CDC};
use usb_device::prelude::{UsbDevice, UsbDeviceBuilder, UsbVidPid};
use usb_device::bus::UsbBusAllocator;


macro_rules! handle_usb {
    ($usb_device: expr, $usb_serial: expr) => {
        if $usb_device.poll(&mut [&mut *$usb_serial]) {
            let mut buf = [0u8; 8];

            if let Ok(count) =  $usb_serial.read(&mut buf) {
                if count > 0 {
                    if let Ok(data) = str::from_utf8(&buf[0..count]) {
                        if data.contains("hi") {
                            $usb_serial.write(b"hello").ok();
                        }
                    }
                }
            }
        }
    };
}

#[app(device = stm32f1xx_hal::stm32)]
const APP: () = {
    static mut USB_DEVICE: UsbDevice<'static, UsbBusType> = ();
    static mut USB_SERIAL: SerialPort<'static, UsbBusType> = ();

    #[init]
    fn init() {
        static mut USB_BUS: Option<UsbBusAllocator<UsbBusType>> = None;

        let mut flash = device.FLASH.constrain();
        let mut rcc = device.RCC.constrain();
        let mut gpioc = device.GPIOC.split(&mut rcc.apb2);
        let mut gpioa = device.GPIOA.split(&mut rcc.apb2);

        let clocks = rcc
            .cfgr
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

        *USB_BUS = Some(UsbBus::new(device.USB,
                                    (usb_data_minus, usb_data_plus)));
        let usb_serial = SerialPort::new(USB_BUS.as_ref().unwrap());
        let usb_device = UsbDeviceBuilder::new(USB_BUS.as_ref().unwrap(),
            UsbVidPid(0x1337, 0x1337))
            .manufacturer("Completely Managed")
            .product("Sensor Board")
            .serial_number("0001")
            .device_class(USB_CLASS_CDC)
            .build();

        USB_DEVICE = usb_device;
        USB_SERIAL = usb_serial;
    }


    #[interrupt(resources = [USB_DEVICE, USB_SERIAL])]
    fn USB_HP_CAN_TX() {
        handle_usb!(resources.USB_DEVICE, resources.USB_SERIAL);
    }

    #[interrupt(resources = [USB_DEVICE, USB_SERIAL])]
    fn USB_LP_CAN_RX0() {
        handle_usb!(resources.USB_DEVICE, resources.USB_SERIAL);
    }

};
