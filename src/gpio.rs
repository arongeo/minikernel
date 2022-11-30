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
struct Pin {
    id: u8,
    function: PinFunction,
    status: PinStatus,
    function_changed: bool,
    status_changed: bool,
}

impl Pin {
    fn new(id_arg: u8) -> Self {
        Self {
            id: id_arg,
            function: PinFunction::Input,
            status: PinStatus::Off,
            function_changed: false,
            status_changed: false,
        }
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

    fn gpio_func_sel(&mut self, pin: &mut Pin) {

        let mut func_sel_reg_addr = BASE_GPIO_ADDR + ((pin.id as u32) / 10) * 4;

        let mut func_sel_mask: u32 = 0b111; 
        let pin_bit_num = pin.id % 10;
        func_sel_mask = func_sel_mask << pin_bit_num*3;

        let mut func_sel_reg_val: u32 = 0;
        unsafe {
            func_sel_reg_val = core::ptr::read_volatile(func_sel_reg_addr as *mut u32);
        };

        if pin.id == 24 {
            
        }

        func_sel_reg_val &= !(func_sel_mask);

        func_sel_reg_val |= 1 << pin_bit_num*3;

        unsafe {
            core::ptr::write_volatile(func_sel_reg_addr as *mut u32, func_sel_reg_val);
        };   
    }

    fn gpio_set(&mut self, pin: &mut Pin) -> Result<(), ErrorCode> {
        let mut gpio_reg_addr: u32 = 0;

        match pin.status {
            PinStatus::Off  => gpio_reg_addr = BASE_GPIO_ADDR + 0x28,
            PinStatus::On   => gpio_reg_addr = BASE_GPIO_ADDR + 0x1c,
            _               => return Err(ErrorCode::GPIOStatusUnwriteable),
        };


        gpio_reg_addr = gpio_reg_addr + ((pin.id as u32) / 32) * 4;
        let mut gpio_mask: u32 = 1 << pin.id;

        let mut gpio_reg_val: u32 = 0;
        unsafe {
            gpio_reg_val = core::ptr::read_volatile(gpio_reg_addr as *mut u32);
        }

        gpio_reg_val &= !(gpio_mask);
        gpio_reg_val |= 1 << pin.id;

        unsafe {
            core::ptr::write_volatile(gpio_reg_addr as *mut u32, gpio_reg_val);
        }
        Ok(())
    }

    fn render_changes(&mut self, pin: &mut Pin) {
        if pin.function_changed == true {
            self.gpio_func_sel(pin);
            pin.function_changed = false;
        }
        if pin.status_changed == true {
            loop {
                if self.gpio_set(pin).is_err() == true {
                    pin.status = PinStatus::Off;
                } else {
                    break;
                }
            }
            pin.status_changed = false;
        }
    }

    pub fn set_function(&mut self, pin_number: usize, function: PinFunction) {
        if pin_number < 58 {
            let mut pin = self.pins[pin_number];
            if pin.function != function {
                pin.function = function;
                pin.function_changed = true;
            }
            if (function == PinFunction::Input) && (pin.status != PinStatus::Off) {
                pin.status = PinStatus::Off;
                pin.status_changed = true;
            }
            self.render_changes(&mut pin);
        }
    }

    pub fn get_function(&mut self, pin_number: usize) -> PinFunction {
        if pin_number < 58 {
            self.pins[pin_number].function
        } else {
            PinFunction::NoFunc
        }
    }

    pub fn set_status(&mut self, pin_number: usize, status_arg: PinStatus) {
        if pin_number < 58 {
            let mut pin = self.pins[pin_number];
            //if (pin.function == PinFunction::Output) & (pin.status != status_arg) {
            pin.status = status_arg;
            pin.status_changed = true;
            //}
            self.render_changes(&mut pin);
        }
    }

    pub fn get_status(&mut self, pin_number: usize) -> PinStatus {
        if pin_number < 58 {
            self.pins[pin_number].status
        } else {
            PinStatus::NoStat
        }
    }
}
