
use crate::gpio;
use crate::uart;
use core::panic::PanicInfo;

#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    let mut gpio_interface = gpio::GPIO::new();
    let mut uart_interface = match uart::MiniUART::new(&mut gpio_interface, 14, 15) {
        Err(error)  => { loop {} },
        Ok(uart_if) => uart_if,
    };
    loop {}
}
