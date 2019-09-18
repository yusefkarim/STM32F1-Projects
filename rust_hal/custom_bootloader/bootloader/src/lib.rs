#![deny(unsafe_code)]
#![no_std]

#[macro_export]
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
