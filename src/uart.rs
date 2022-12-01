#[path = "config.rs"]
mod config;
use config::*;

// AUX Registers that are important for miniUART, this enum might expand and relocate to another
// file if I end up adding SPI.
enum AuxRegisters {
    AUX_IRQ         =   BASE_AUX_ADDR,
    AUX_ENABLES     =   BASE_AUX_ADDR + 0x04,
    AUX_MU_IO_REG   =   BASE_AUX_ADDR + 0x40,
    AUX_MU_IER_REG  =   BASE_AUX_ADDR + 0x44,
    AUX_MU_IIR_REG  =   BASE_AUX_ADDR + 0x48,
    AUX_MU_LCR_REG  =   BASE_AUX_ADDR + 0x4c,
}
