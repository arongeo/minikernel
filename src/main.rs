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
    uart_conn.write_to_uart("It works, and the RPi 4 datasheets aren't correct.");

    loop {}
}
