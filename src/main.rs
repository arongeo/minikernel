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
    
    gpio_pins.set_function(21, PinFunction::Output);
    gpio_pins.set_function(24, PinFunction::Output);

    if gpio_pins.get_function(24) == PinFunction::Input {
        gpio_pins.set_status(24, PinStatus::On);
    }

    loop {}
}
