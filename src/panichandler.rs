
use core::panic::PanicInfo;
use crate::drivers;

#[panic_handler]
pub fn panic(panic_info: &PanicInfo) -> ! {
    let mut mini_uart: &mut crate::drivers::MiniUART;
    unsafe {
        mini_uart = drivers::get_mini_uart().unwrap();
    }
    mini_uart.write_to_uart(panic_info.payload().downcast_ref::<&str>().unwrap());
    loop {}
}
