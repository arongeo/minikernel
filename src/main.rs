#![no_std]
#![no_main]
#![allow(dead_code)]

#[path = "./boot/boot.rs"]
mod boot;
#[path = "panichandler.rs"]
mod panichandler;
#[path = "errorcodes.rs"]
mod errorcodes;
#[path = "drivers.rs"]
mod drivers;
#[path = "macros.rs"]
pub mod macros;

use errorcodes::ErrorCode;
use drivers::gpio::PinFunction;
use drivers::gpio::PinStatus;

pub fn kernel_start() -> ! {
    let gpio_pins: &mut drivers::GPIO;
    unsafe {
        gpio_pins = drivers::get_gpio_handler().unwrap();
    }
 
    uart_println!("Control write");

    gpio_pins.get_pin(21).unwrap().set_function(PinFunction::Output);
    gpio_pins.get_pin(21).unwrap().set_status(PinStatus::On);

    panic!("uh oh");
}
