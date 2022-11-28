#![no_std]
#![no_main]

#[path = "./boot/boot.rs"]
mod boot;
#[path = "panichandler.rs"]
mod panichandler;
#[path = "gpio.rs"]
mod gpio;

pub unsafe fn kernel_start() -> ! {
    core::ptr::write_volatile(0xFE20_0008 as *mut u32, 1<<3);

    loop {
        core::ptr::write_volatile(0xFE20_001c as *mut u32, 1<<21);

        for _ in 0..50000 {
            core::arch::asm!("nop");
        }

        core::ptr::write_volatile(0xFE20_0028 as *mut u32, 1<<21);

        for _ in 0..50000 {
            core::arch::asm!("nop");
        }
    }
}
