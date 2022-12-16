
use core::panic::PanicInfo;
use crate::drivers;

#[panic_handler]
pub fn panic(panic_info: &PanicInfo) -> ! {
    let mut mini_uart: &mut crate::drivers::MiniUART;
    unsafe {
        mini_uart = match drivers::get_mini_uart() {
            Ok(m_uart) => m_uart,
            Err(error) => loop {},
        };
    }
    let mut panic_message: &str = match panic_info.payload().downcast_ref::<&str>() {
        Some(message) => message,
        None => "PANIC: Couldn't find panic message.",
    };
    mini_uart.write_str(panic_message);
    loop {}
}
