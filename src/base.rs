//! base.rs

pub const PERIPHERAL_BASE: u32 = 0x20000000;

pub unsafe fn get_reg(base: *const u32, offset: isize) -> *mut u32 {
   base.offset(offset) as *mut u32
}
    
pub fn nothing() {
    unsafe { asm!(""); }
}
