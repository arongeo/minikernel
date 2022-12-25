//  
//! miniUART print macro module
//  copyright 2022 - arongeo
//  https://arongeo.com
//

pub fn _print(text: &str) {
    let mini_uart: &mut crate::drivers::MiniUART = match crate::drivers::get_mini_uart() {
        Ok(m_uart) => m_uart,
        Err(_error) => panic!("PANIC: ERROR: _print failed to get miniUART handler"),
    };
    mini_uart.write_str(text);
}

#[macro_export]
macro_rules! uart_print {
    ($($arg:tt)*) => {{
        $crate::macros::_print($($arg)*);
    }};
}

#[macro_export]
macro_rules! uart_println {
    () => ($crate::uart_print!("\n"));
    ($($arg:tt)*) => {{
        $crate::macros::_print($($arg)*);
    }};
}
