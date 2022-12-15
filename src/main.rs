#![no_std]
#![no_main]

#[path = "./boot/boot.rs"]
mod boot;
#[path = "panichandler.rs"]
mod panichandler;
#[path = "errorcodes.rs"]
mod errorcodes;
#[path = "drivers.rs"]
mod drivers;

use errorcodes::ErrorCode;
use drivers::gpio::PinFunction;
use drivers::gpio::PinStatus;

pub fn kernel_start() -> ! {
    let mut mini_uart: &mut drivers::MiniUART;
    let mut gpio_pins: &mut drivers::GPIO;
    unsafe {
        mini_uart = drivers::get_mini_uart().unwrap();
        gpio_pins = drivers::get_gpio_handler().unwrap();
    }

    gpio_pins.get_pin(21).unwrap().set_function(drivers::gpio::PinFunction::Output);
    gpio_pins.get_pin(21).unwrap().set_status(drivers::gpio::PinStatus::On);

    mini_uart.init();
    mini_uart.write_to_uart("asd from main");

    loop {}
}
