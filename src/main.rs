#![no_std]
#![no_main]

#[path = "./boot/boot.rs"]
mod boot;
#[path = "panichandler.rs"]
mod panichandler;
#[path = "gpio.rs"]
mod gpio;

use gpio::PinFunction;
use gpio::PinStatus;

pub unsafe fn kernel_start() -> ! {
    let mut gpio_pins = gpio::GPIO::new();

    gpio_pins.get_pin(24).unwrap().set_function(PinFunction::Output);
    gpio_pins.get_pin(24).unwrap().set_status(PinStatus::On);

    loop {}
}
