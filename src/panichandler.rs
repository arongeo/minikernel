
use core::panic::PanicInfo;
use crate::drivers;

#[panic_handler]
pub unsafe fn panic(panic_info: &PanicInfo) -> ! {
    let mut mini_uart: &mut crate::drivers::MiniUART = drivers::get_mini_uart().unwrap();
    mini_uart.write_to_uart("asd from panic");
    loop {}
}
