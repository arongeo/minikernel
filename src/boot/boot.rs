use core::arch::global_asm;

#[no_mangle]
#[link_section = ".text._start_args"]
pub static MASTER_CORE_ID: u64 = 0;

global_asm!(
    include_str!("boot.s")
);

#[no_mangle]
pub unsafe fn _rust_entry() -> ! {
    crate::drivers::init();
    crate::kernel_start();
}
