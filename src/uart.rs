//
//! UART access module
//  copyright 2022 - arongeo
//  https://arongeo.com
//

#[path = "config.rs"]
mod config;
#[path = "mem.rs"]
mod mem;

use config::*;
use crate::drivers::gpio;
use crate::ErrorCode;

// AUX Registers that are important for miniUART, this enum might expand and relocate to another
// file if I end up adding SPI.
#[allow(non_camel_case_types)]
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
    UART_QUEUE_MAX  =   16 * 1024,
}

static mut UART_OUTPUT_QUEUE: [u32; AuxRegisters::UART_QUEUE_MAX as usize] = [0; AuxRegisters::UART_QUEUE_MAX as usize];
static mut UART_OUTPUT_QUEUE_WRITE_COUNT: usize = 0;
static mut UART_OUTPUT_QUEUE_READ_COUNT: usize = 0;     

fn insert_bits(mut val: u32, left_shift_by: u32, bits_length: u32, mut inserted_bit_val: u32) -> u32 {
    inserted_bit_val = inserted_bit_val << left_shift_by;
    let bit_mask = (u32::pow(2, bits_length) - 1) << left_shift_by;
    val &= !(bit_mask);
    val |= inserted_bit_val;
    val
}

fn calc_baudrate_divisor(baud: u32) -> u32 {
    (500_000_000/(8*baud))-1
}


pub struct MiniUART {
    tx_pin: gpio::Pin,
    rx_pin: gpio::Pin,
    output_queue: [char; AuxRegisters::UART_QUEUE_MAX as usize],
    output_queue_read: usize,
    output_queue_write: usize,
}

impl MiniUART {
    pub fn new(gpio_pins: &mut gpio::GPIO, tx_pin_num: usize, rx_pin_num: usize) -> Result<Self, ErrorCode> {
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
            output_queue: ['\0'; AuxRegisters::UART_QUEUE_MAX as usize],
            output_queue_read: 0,
            output_queue_write: 0,
        })
    }

    pub fn init(&mut self) {
        mem::write_addr_val(AuxRegisters::AUX_ENABLES as u32, 1);
        mem::write_addr_val(AuxRegisters::AUX_MU_IER_REG as u32, 0);
        mem::write_addr_val(AuxRegisters::AUX_MU_CNTL_REG as u32, 0);
        mem::write_addr_val(AuxRegisters::AUX_MU_LCR_REG as u32, 0b11);
        mem::write_addr_val(AuxRegisters::AUX_MU_MCR_REG as u32, 0);
        mem::write_addr_val(AuxRegisters::AUX_MU_IER_REG as u32, 0);
        mem::write_addr_val(AuxRegisters::AUX_MU_IIR_REG as u32, insert_bits(mem::read_addr_val(AuxRegisters::AUX_MU_IIR_REG as u32), 1, 2, 0b11));
        mem::write_addr_val(AuxRegisters::AUX_MU_BAUD_REG as u32, calc_baudrate_divisor(115200));
        self.tx_pin.set_function(gpio::PinFunction::Alt5);
        self.rx_pin.set_function(gpio::PinFunction::Alt5);
        mem::write_addr_val(AuxRegisters::AUX_MU_CNTL_REG as u32, 0b11);
    }

    fn is_fifo_readable(&mut self) -> bool {
        if (mem::read_addr_val(AuxRegisters::AUX_MU_LSR_REG as u32) & 0b1) == 1 {
            return true;
        } else {
            return false;
        }
    }

    fn is_fifo_writable(&mut self) -> bool {
        if (mem::read_addr_val(AuxRegisters::AUX_MU_LSR_REG as u32) & 0b100000) == 32 {
            return true;
        } else {
            return false;
        }
    }

    fn write_char(&mut self, character: char) {
        while self.is_fifo_writable() == false {}
        mem::write_addr_val(AuxRegisters::AUX_MU_IO_REG as u32, character as u32);
    }

    fn read_char(&mut self) -> char {
        while self.is_fifo_readable() == false {}
        (mem::read_addr_val(AuxRegisters::AUX_MU_IO_REG as u32) as u8) as char
    }

    pub fn write_str(&mut self, text: &str) {
        for character in text.chars() {
            if character == '\n' {
                self.write_char('\r');
            }
            self.write_char(character);
        }
    }

    fn write_char_back(&mut self, character: char) {
        self.write_char(character);
    } 

    fn write_to_queue(&mut self, character: char) {
        unimplemented!();
    }

    pub fn wait_for_string(&mut self) -> UARTString {
        let mut char_buffer: [u8; 512] = ['\0' as u8; 512];
        let mut char_buffer_pointer: usize = 0;
        while (char_buffer[char_buffer_pointer] != '\r' as u8) && (char_buffer_pointer < 512) {
            let curr_char = self.read_char();
            char_buffer[char_buffer_pointer] = curr_char as u8;
            char_buffer_pointer = char_buffer_pointer + 1;
            self.write_char_back(curr_char)
        }
        UARTString::from(char_buffer)
    }
}
