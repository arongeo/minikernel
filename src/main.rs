#![no_std]
#![no_main]

#[path = "./boot/boot.rs"]
mod boot;
#[path = "panichandler.rs"]
mod panichandler;
#[path = "errorcodes.rs"]
mod errorcodes;
#[path = "gpio.rs"]
mod gpio;
#[path = "uart.rs"]
mod uart;

use errorcodes::ErrorCode;
use gpio::PinFunction;
use gpio::PinStatus;

pub unsafe fn kernel_start() -> ! {
    let mut gpio_pins = gpio::GPIO::new();

    let mut uart_conn = uart::MiniUART::new(&mut gpio_pins, 14, 15).unwrap();
    uart_conn.init();
    uart_conn.write_to_uart("Hello World!\n");

    gpio_pins.get_pin(24).unwrap().set_function(PinFunction::Output);
    gpio_pins.get_pin(21).unwrap().set_function(PinFunction::Output);

    loop {   
        gpio_pins.get_pin(24).unwrap().set_status(PinStatus::On);
        gpio_pins.get_pin(21).unwrap().set_status(PinStatus::On);

        for _ in 0..50000 {
            core::arch::asm!("nop");
        }

        gpio_pins.get_pin(24).unwrap().set_status(PinStatus::Off);
        gpio_pins.get_pin(21).unwrap().set_status(PinStatus::Off);

        for _ in 0..50000 {
            core::arch::asm!("nop");
        }
    }
}
