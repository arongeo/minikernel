//! GPIO Access Module

#[path = "config.rs"]
mod config;
use config::*;

#[path = "mem.rs"]
mod mem;

use crate::ErrorCode;

#[derive(Copy, Clone, PartialEq)]
pub enum PinFunction {
    Input,
    Output,
    Alt0,
    Alt1,
    Alt2,
    Alt3,
    Alt4,
    Alt5,
}

#[derive(Copy, Clone, PartialEq)]
pub enum PullState {
    Off,
    PullDown,
    PullUp,
}

#[derive(Copy, Clone, PartialEq)]
pub enum PinStatus {
    On,
    Off,
}

#[derive(Copy, Clone, PartialEq)]
pub enum PinUsage {
    GPIOUsage,
    UARTUsage,
}

#[derive(Copy, Clone, PartialEq)]
pub struct Pin {
    id:         u8,
    function:   PinFunction,
    status:     PinStatus,
    usage:      PinUsage,
    pullstate:  PullState,
}

impl Pin {
    pub fn new(id_arg: u8) -> Self {
        Self {
            id:         id_arg,
            function:   PinFunction::Input,
            status:     PinStatus::Off,
            usage:      PinUsage::GPIOUsage,
            pullstate:  PullState::Off,
        }
    }

    pub fn get_status(&mut self) -> PinStatus {
        self.status
    }

    pub fn get_function(&self) -> PinFunction {
        self.function
    }

    pub fn set_status(&mut self, status_arg: PinStatus) -> Result<(), ErrorCode> {
        if self.function == PinFunction::Output {
            self.status = status_arg;
            self.gpio_set();
            return Ok(());
        } else {
            return Err(ErrorCode::GPIOPinWrongFunction);
        }
    }

    pub fn set_function(&mut self, function_arg: PinFunction) {
        self.function = function_arg;
        self.gpio_func_sel();
    }

    fn gpio_func_sel(&mut self) {
        let func_sel_reg_addr = BASE_GPIO_ADDR + ((self.id as u32) / 10) * 4;

        let mut func_sel_mask: u32 = 0b111; 
        let pin_bit_num = self.id % 10;
        func_sel_mask = func_sel_mask << pin_bit_num*3;

        let mut func_sel_reg_val: u32 = mem::read_addr_val(func_sel_reg_addr);

        func_sel_reg_val &= !(func_sel_mask);

        match self.function {
            PinFunction::Output => func_sel_reg_val |= 0b001 << pin_bit_num*3,
            PinFunction::Alt0   => func_sel_reg_val |= 0b100 << pin_bit_num*3,
            PinFunction::Alt5   => func_sel_reg_val |= 0b010 << pin_bit_num*3,
            _                   => func_sel_reg_val |= 0b001 << pin_bit_num*3,
        }
        
        mem::write_addr_val(func_sel_reg_addr, func_sel_reg_val);
    }

    fn gpio_set(&mut self) {
        let mut gpio_reg_addr: u32 = match self.status {
            PinStatus::Off  => BASE_GPIO_ADDR + 0x28,
            PinStatus::On   => BASE_GPIO_ADDR + 0x1c,
        };

        gpio_reg_addr = gpio_reg_addr + ((self.id as u32) / 32) * 4;
        let gpio_mask: u32 = 1 << self.id;

        let mut gpio_reg_val: u32 = mem::read_addr_val(gpio_reg_addr);

        gpio_reg_val &= !(gpio_mask);
        gpio_reg_val |= 1 << self.id;

        mem::write_addr_val(gpio_reg_addr, gpio_reg_val);
    }

    fn pull_set(&mut self) {
        let gpio_pull_reg_addr = BASE_GPIO_ADDR + 0xE4 + (((self.id as u32) / 16) * 4);

        let mut gpio_pull_reg_val: u32 = mem::read_addr_val(gpio_pull_reg_addr);
        let mut gpio_pull_mask: u32 = 0b11;
        let pin_bit_num = (self.id % 16) * 2;
        gpio_pull_mask = gpio_pull_mask << pin_bit_num;

        gpio_pull_reg_val &= !(gpio_pull_mask);

        match self.pullstate {
            PullState::PullUp   => gpio_pull_reg_val |= 0b01 << pin_bit_num,
            PullState::PullDown => gpio_pull_reg_val |= 0b11 << pin_bit_num,
            PullState::Off      => (),
        }

        mem::write_addr_val(gpio_pull_reg_addr, gpio_pull_reg_val);
    }

    pub fn get_usage(&mut self) -> PinUsage {
        self.usage
    }
    
    pub fn set_usage(&mut self, usage: PinUsage) {
        self.usage = usage;
    }

    pub fn get_pullstate(&mut self) -> PullState {
        self.pullstate
    }

    pub fn set_pullstate(&mut self, pullstate: PullState) -> Result<(), ErrorCode> {
        if (self.function == PinFunction::Input) | (self.function == PinFunction::Output) {
            return Err(ErrorCode::GPIOPinWrongFunction);
        } else {
            self.pullstate = pullstate;
            self.pull_set();
            return Ok(());
        }
    }
}

#[derive(Copy, Clone)]
pub struct GPIO {
    pins: [Pin; 58],
}

impl GPIO {
    pub fn new() -> Self {
        let mut pin_buffer = [Pin::new(0); 58];
        for i in 0..58 {
            pin_buffer[i] = Pin::new(i as u8);
        }
        Self {
            pins: pin_buffer,
        }
    }

    pub fn get_pin(&mut self, id: usize) -> Result<&mut Pin, ErrorCode> {
        if id < 58 {
            if self.pins[id].get_usage() == PinUsage::GPIOUsage {
                return Ok(&mut self.pins[id]);
            } else {
                return Err(ErrorCode::GPIOPinUsedByOtherProcess)
            }
        } else {
            return Err(ErrorCode::GPIOPinOutOfBounds);
        }
    }
}
