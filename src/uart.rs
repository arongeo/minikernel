#[path = "config.rs"]
mod config;
use config::*;

#[path = "gpio.rs"]
mod gpio;

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
