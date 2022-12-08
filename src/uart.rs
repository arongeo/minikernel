//! UART Access Module

#[path = "config.rs"]
mod config;
#[path = "mem.rs"]
mod mem;

use config::*;
use crate::gpio;
use crate::ErrorCode;

// AUX Registers that are important for miniUART, this enum might expand and relocate to another
// file if I end up adding SPI.
#[repr(u32)]
enum AuxRegisters {
    AUX_IRQ         =   BASE_AUX_ADDR,
    AUX_ENABLES     =   BASE_AUX_ADDR + 0x04,
    AUX_MU_IO_REG   =   BASE_AUX_ADDR + 0x40,
    AUX_MU_IER_REG  =   BASE_AUX_ADDR + 0x44,
    AUX_MU_IIR_REG  =   BASE_AUX_ADDR + 0x48,
    AUX_MU_LCR_REG  =   BASE_AUX_ADDR + 0x4c,
    AUX_MU_MCR_REG  =   BASE_AUX_ADDR + 0x50,
    AUX_MU_LSR_REG  =   BASE_AUX_ADDR + 0x54,
    AUX_MU_MSR_REG  =   BASE_AUX_ADDR + 0x58,
    AUX_MU_SCRATCH  =   BASE_AUX_ADDR + 0x5c,
    AUX_MU_CNTL_REG =   BASE_AUX_ADDR + 0x60,
    AUX_MU_STAT_REG =   BASE_AUX_ADDR + 0x64,
    AUX_MU_BAUD_REG =   BASE_AUX_ADDR + 0x68,
}

const BAUD_RATE: u32 = 1; 

pub struct MiniUART {
    tx_pin: gpio::Pin,
    rx_pin: gpio::Pin,
}

impl MiniUART {
    pub fn new(gpio_pins: &mut crate::gpio::GPIO, tx_pin_num: usize, rx_pin_num: usize) -> Result<Self, ErrorCode> {
        match gpio_pins.get_pin(tx_pin_num) {
            Err(error)  => return Err(error),
            Ok(result)  => result.set_usage(gpio::PinUsage::UARTUsage),
        }

        match gpio_pins.get_pin(rx_pin_num) {
            Err(error)  => return Err(error),
            Ok(result)  => result.set_usage(gpio::PinUsage::UARTUsage),
        }


        Ok(Self {
            tx_pin: gpio::Pin::new(tx_pin_num as u8),
            rx_pin: gpio::Pin::new(rx_pin_num as u8),
        })
    }

    pub fn init(&mut self) {
        self.tx_pin.set_function(gpio::PinFunction::Alt0);
        self.rx_pin.set_function(gpio::PinFunction::Alt0);

        mem::write_addr_val(AuxRegisters::AUX_ENABLES as u32, 1);
    }
}
