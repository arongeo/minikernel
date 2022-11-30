#[path = "config.rs"]
mod config;
use config::addresses::*;

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

    fn render_changes(&mut self) {
        for pin in self.pins.iter_mut() {
            if pin.function_changed == true {
                let mut func_sel_reg_addr = BASE_GPIO_ADDR + ((pin.id as u32) / 10) * 4;

                let mut func_sel_mask: u32 = 0b111; 
                let pin_bit_num = pin.id % 10;
                func_sel_mask = func_sel_mask << pin_bit_num*3;

                let mut func_sel_reg_val: u32 = 0;
                unsafe {
                    func_sel_reg_val = core::ptr::read_volatile(func_sel_reg_addr as *mut u32);
                };

                func_sel_reg_val &= !(func_sel_mask);

                if pin.function == PinFunction::Output {
                    func_sel_reg_val |= 1 << pin_bit_num*3;
                }

                unsafe {
                    core::ptr::write_volatile(func_sel_reg_addr as *mut u32, func_sel_reg_val);
                };
                pin.function_changed = false;
            }
            if pin.status_changed == true {
                if pin.status == PinStatus::Off {
                    // The pin status is off, so we have to write to one of the GPCLR registers
                    let gpclr_reg_addr: u32 = BASE_GPIO_ADDR + 0x28 + (((pin.id / 32) * 4) as u32);
                    let mut gpclr_mask: u32 = 1 << pin.id;

                    let mut gpclr_reg_val: u32 = 0;
                    unsafe {
                        gpclr_reg_val = core::ptr::read_volatile(gpclr_reg_addr as *mut u32);
                    }

                    gpclr_reg_val &= !(gpclr_mask);
                    gpclr_reg_val |= 1 << pin.id;

                    unsafe {
                        core::ptr::write_volatile(gpclr_reg_addr as *mut u32, gpclr_reg_val);
                    }
                    pin.status_changed = false;
                } 
            }
        }
    }

    pub fn set_function(&mut self, pin_number: usize, function: PinFunction) {
        if pin_number < 58 {
            if self.pins[pin_number].function != function {
                self.pins[pin_number].function = function;
                self.pins[pin_number].function_changed = true;
            }
        }
        if (function == PinFunction::Input) & (self.pins[pin_number].status != PinStatus::Off) {
            self.pins[pin_number].status = PinStatus::Off;
            self.pins[pin_number].status_changed = true;
        }
    }

    pub fn get_function(&mut self, pin_number: usize) -> PinFunction {
        if pin_number < 58 {
            self.pins[pin_number].function
        } else {
            PinFunction::NoFunc
        }
    }

    pub fn set_status(&mut self, pin_number: usize, status: PinStatus) {
        if pin_number < 58 {
            if (self.pins[pin_number].function == PinFunction::Output) & (self.pins[pin_number].status != status) {
                self.pins[pin_number].status = status;
            }
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
