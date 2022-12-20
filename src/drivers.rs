#[path = "uart.rs"]
pub mod uart;
#[path = "gpio.rs"]
pub mod gpio;

pub use gpio::GPIO;
pub use uart::MiniUART;
use crate::ErrorCode;

static mut GPIO_PINS: Option<GPIO>      = None;
static mut MINI_UART: Option<MiniUART>  = None;

pub fn init() -> Result<(), ErrorCode> {
    let mut gpio_pins: GPIO = GPIO::new();
    let mut mini_uart: MiniUART = match MiniUART::new(&mut gpio_pins, 14, 15) {
        Ok(m_uart) => m_uart,
        Err(error) => return Err(error),
    };
    mini_uart.init();
    unsafe {
        GPIO_PINS = Some(gpio_pins);
        MINI_UART = Some(mini_uart);
    };
    Ok(())
}

pub fn get_mini_uart() -> Result<&'static mut MiniUART, ErrorCode> {
    unsafe {
        match &mut MINI_UART {
            Some(mini_uart) => return Ok(mini_uart),
            None => return Err(ErrorCode::DRIVERSMiniUARTInaccessible),
        }
    }
}

pub fn get_gpio_handler() -> Result<&'static mut GPIO, ErrorCode> {
    unsafe {
        match &mut GPIO_PINS {
            Some(gpio_pins) => return Ok(gpio_pins),
            None => return Err(ErrorCode::DRIVERSGPIOHandlerInaccessible),
        }
    }
}
