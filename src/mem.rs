//! Memory Operations

pub fn read_mem_addr_val(addr: u32) -> u32 {
    let mut memory_addr_val: u32 = 0;
    unsafe {
        memory_addr_val = core::ptr::read_volatile(addr as *mut u32);
    }
    memory_addr_val
}

pub fn write_mem_addr_val(addr: u32, val: u32) {
    unsafe {
        core::ptr::write_volatile(addr as *mut u32, val);
    }
}
