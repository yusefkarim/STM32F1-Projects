#![deny(unsafe_code)]
#![no_std]
#![no_main]
extern crate panic_halt;

use rtfm::app;
use heapless::{
    consts::*,
    i,
    spsc::{Queue, Consumer, Producer},
};
use cortex_m::asm::wfi;
use embedded_hal::digital::v2::OutputPin;

#[app(device = stm32f1xx_hal::stm32, peripherals = true)]
const APP: () = {
    struct Resources {
        uart3_p: Producer<'static, u8, U80>,
        uart3_c: Consumer<'static, u8, U80>,
        usb_p: Producer<'static, u8, U40>,
        usb_c: Consumer<'static, u8, U40>,
    }

    #[init]
    fn init(c: init::Context) -> init::LateResources {
        static mut uart3_queue: Queue<u8, U80> = Queue(i::Queue::new());
        static mut usb_queue: Queue<u8, U40> = Queue(i::Queue::new());

        let (mut uart3_p, mut uart3_c) = uart3_queue.split();
        let (mut usb_p, mut usb_c) = usb_queue.split();



        init::LateResources{uart3_p,
                            uart3_c,
                            usb_p,
                            usb_c,}
    }

    #[idle(resources = [uart3_c, usb_c])]
    fn idle(c: idle::Context) -> ! {
        loop {
            if let Some(data) = c.resources.uart3_c.dequeue() {
                // Got uart3 data
            } else if let Some(data) = c.resources.usb_c.dequeue() {
                // Got usb data
            } else {
                wfi();
            }
        }
    }

    #[task(binds = USART3, resources = [uart3_p])]
    fn uart3(mut c: uart3::Context) {
        // Do stuff here, get the data, then enqueue below
        if let Err(data) = c.resources.uart3_p.enqueue(b'h') {
            // Queue is full, what should we do with the data?
        }
    }

    #[task(binds = USB_HP_CAN_TX, resources = [usb_p])]
    fn usb_hp(mut c: usb_hp::Context) {
        // Do stuff here, get the data, then enqueue below
        if let Err(data) = c.resources.usb_p.enqueue(b'h') {
            // Queue is full, what should we do with the data?
        }
    }

    #[task(binds = USB_LP_CAN_RX0, resources = [usb_p])]
    fn usb_lp(mut c: usb_lp::Context) {
        // Do stuff here, get the data, then enqueue below
        if let Err(data) = c.resources.usb_p.enqueue(b'h') {
            // Queue is full, what should we do with the data?
        }
    }

};
