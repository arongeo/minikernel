//
//! panic handler module
//  copyright 2022 - arongeo
//  https://arongeo.com
//

use core::panic::PanicInfo;
use crate::drivers;

#[panic_handler]
pub fn panic(panic_info: &PanicInfo) -> ! {
    let mini_uart: &mut crate::drivers::MiniUART = match drivers::get_mini_uart() {
        Ok(m_uart) => m_uart,
        Err(_error) => loop {},
    };
    let panic_message: &str = match panic_info.payload().downcast_ref::<&str>() {
        Some(message) => message,
        None => "PANIC: Couldn't find panic message.",
    };
    mini_uart.write_str(panic_message);
    loop {}
}
