#[cfg(any(peri_mode = "low", doc))]
pub const BASE_PERI_ADDR: u32 = 0xFE00_0000;

#[cfg(peri_mode = "high")]
pub const BASE_PERI_ADDR: u32 = 0x7E00_0000;

pub const BASE_GPIO_ADDR: u32 = BASE_PERI_ADDR + 0x200000;
pub const BASE_AUX_ADDR: u32 = BASE_PERI_ADDR + 0x215000;
