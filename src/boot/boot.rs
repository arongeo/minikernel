//
//! rust boot module - entry into rust 
//  copyright 2022 - arongeo
//  https://arongeo.com
//

use core::arch::global_asm;

#[no_mangle]
#[link_section = ".text._start_args"]
pub static MASTER_CORE_ID: u64 = 0;

global_asm!(
    include_str!("boot.s")
);

#[no_mangle]
pub unsafe fn _rust_entry() -> ! {
    match crate::drivers::init() {
        Ok(_val) => (),
        Err(_error) => loop {},
    };
    crate::kernel_start();
}
