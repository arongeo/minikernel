#![no_std]
#![no_main]

#[path = "./aarch64/boot/boot.rs"]
mod boot;

#[path = "panic_fn.rs"]
mod panic_fn;

pub unsafe fn kernel_start() -> ! {
    core::arch::asm!("wfe");
    loop {}
}
