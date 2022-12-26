//
//! configuration module - definitions of important constant values
//  copyright 2022 - arongeo
//  https://arongeo.com
//

#[cfg(any(peri_mode = "low", doc))]
pub const BASE_PERI_ADDR: u32 = 0xFE00_0000;

#[cfg(peri_mode = "high")]
pub const BASE_PERI_ADDR: u32 = 0x7E00_0000;

pub const BASE_GPIO_ADDR: u32 = BASE_PERI_ADDR + 0x200000;
pub const BASE_AUX_ADDR: u32 = BASE_PERI_ADDR + 0x215000;

pub struct UARTString {
    string: [u8; 1024],
    chars_written: u32,
}

impl UARTString {
    pub fn from(origin: [u8; 1024], ch_written: u32) -> Self {
        Self {
            string: origin,
            chars_written: ch_written,
        }
    }

    pub fn to_str(&mut self) -> Result<&str, core::str::Utf8Error> {
        match core::str::from_utf8(&self.string) {
           Ok(val) => Ok(val.trim_end()),
           Err(error) => Err(error),
        }
    }
}
