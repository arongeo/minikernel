
#[derive(Copy, Clone, PartialEq)]
pub enum PinFunction {
    Input,
    Output,
    NoFunc
}

#[derive(Copy, Clone, PartialEq)]
pub enum PinStatus {
    On,
    Off
}

#[derive(Copy, Clone)]
struct Pin {
    function: PinFunction,
    status: PinStatus,
}

impl Pin {
    fn new() -> Self {
        Self {
            function: PinFunction::Input,
            status: PinStatus::Off,
        }
    }

    fn set_function(&mut self, function_arg: PinFunction) {
        self.function = function_arg;
    }

    fn get_function(&self) -> PinFunction {
        self.function
    }
}

pub struct GPIO {
    pins: [Pin; 58],
}

impl GPIO {
    pub fn new() -> Self {
        Self {
            pins: [Pin::new(); 58],
        }
    }

    fn render_changes(&mut self) {
        loop {}
    }

    pub fn set_function(&mut self, pin_number: usize, function: PinFunction) {
        if (0 <= pin_number) & (pin_number < 58) {
            self.pins[pin_number].function = function;
        }
        if function == PinFunction::Output {
            self.pins[pin_number].status = PinStatus::Off;
        }
    }

    pub fn get_function(&mut self, pin_number: usize) -> PinFunction {
        if (0 <= pin_number) & (pin_number < 58) {
            self.pins[pin_number].function
        } else {
            PinFunction::NoFunc
        }
    }

    pub fn set_status(&mut self, pin_number: usize, status: PinStatus) {
        if (0 <= pin_number) & (pin_number < 58) {
            if (self.pins[pin_number].function == PinFunction::Output) {
                self.pins[pin_number].status = status;
            }
        }
    }
}
