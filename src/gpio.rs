#[path = "config.rs"]
mod config;
use config::*;

#[path = "errorcodes.rs"]
mod errorcodes;
use errorcodes::ErrorCode;

#[derive(Copy, Clone, PartialEq)]
pub enum PinFunction {
    Input,
    Output,
    NoFunc
}

#[derive(Copy, Clone, PartialEq)]
pub enum PinStatus {
    On,
    Off,
    NoStat
}

#[derive(Copy, Clone)]
pub struct Pin {
    id: u8,
    function: PinFunction,
    status: PinStatus,
}

impl Pin {
    fn new(id_arg: u8) -> Self {
        Self {
            id: id_arg,
            function: PinFunction::Input,
            status: PinStatus::Off,
        }
    }

    pub fn get_status(&self) -> PinStatus {
        self.status
    }

    pub fn get_function(&self) -> PinFunction {
        self.function
    }

    pub fn get_id(&self) -> u8 {
        self.id
    }

    pub fn set_status(&mut self, status_arg: PinStatus) -> Result<(), ErrorCode> {
        if self.function == PinFunction::Output {
            self.status = status_arg;
            self.gpio_set();
        } else {
            return Err(ErrorCode::GPIOStatusUnwriteable);
        }
        Ok(())
    }

    pub fn set_function(&mut self, function_arg: PinFunction) -> Result<(), ErrorCode> {
        self.function = function_arg;
        self.gpio_func_sel();
        Ok(())
    }

    fn gpio_func_sel(&mut self) {
        let mut func_sel_reg_addr = BASE_GPIO_ADDR + ((self.id as u32) / 10) * 4;

        let mut func_sel_mask: u32 = 0b111; 
        let pin_bit_num = self.id % 10;
        func_sel_mask = func_sel_mask << pin_bit_num*3;

        let mut func_sel_reg_val: u32 = 0;
        unsafe {
            func_sel_reg_val = core::ptr::read_volatile(func_sel_reg_addr as *mut u32);
        };

        func_sel_reg_val &= !(func_sel_mask);

        if self.function == PinFunction::Output {
            func_sel_reg_val |= 1 << pin_bit_num*3;
        }

        unsafe {
            core::ptr::write_volatile(func_sel_reg_addr as *mut u32, func_sel_reg_val);
        };   
    }

    fn gpio_set(&mut self) -> Result<(), ErrorCode> {
        let mut gpio_reg_addr: u32 = 0;

        match self.status {
            PinStatus::Off  => gpio_reg_addr = BASE_GPIO_ADDR + 0x28,
            PinStatus::On   => gpio_reg_addr = BASE_GPIO_ADDR + 0x1c,
            _               => return Err(ErrorCode::GPIOStatusUnwriteable),
        };


        gpio_reg_addr = gpio_reg_addr + ((self.id as u32) / 32) * 4;
        let mut gpio_mask: u32 = 1 << self.id;

        let mut gpio_reg_val: u32 = 0;
        unsafe {
            gpio_reg_val = core::ptr::read_volatile(gpio_reg_addr as *mut u32);
        }

        gpio_reg_val &= !(gpio_mask);
        gpio_reg_val |= 1 << self.id;

        unsafe {
            core::ptr::write_volatile(gpio_reg_addr as *mut u32, gpio_reg_val);
        }
        Ok(())
    }
}

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
            return Ok(&mut self.pins[id]);
        } else {
            return Err(ErrorCode::GPIOPinOutOfBounds);
        }
    }
}
