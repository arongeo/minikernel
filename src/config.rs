#[cfg(peri_mode = "low")]
pub mod addresses {
    pub const BASE_PERI_ADDR: u32 = 0xFE00_0000;
    pub const BASE_GPIO_ADDR: u32 = BASE_PERI_ADDR + 0x200000;
}

#[cfg(peri_mode = "high")]
pub mod addresses {
    pub const BASE_PERI_ADDR: u32 = 0x7E00_0000;
    pub const BASE_GPIO_ADDR: u32 = BASE_PERI_ADDR + 0x200000;
}
