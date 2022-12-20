

pub fn _print(text: &str) {
    let mini_uart: &mut crate::drivers::MiniUART;
    unsafe {
        mini_uart = crate::drivers::get_mini_uart().unwrap();
    };
    mini_uart.write_to_uart(text);
}

#[macro_export]
macro_rules! uart_print {
    ($($arg:tt)*) => {{
        $crate::macros::_print($($arg)*);
    }};
}

#[macro_export]
macro_rules! uart_println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => {{
        $crate::macros::_print($($arg)*);
    }};
}
