//! systimer.rs
//!
//! The System Timer

#![allow(dead_code)]

use base;

pub const SYSTIMER_BASE: u32 = base::PERIPHERAL_BASE + 0x3000;

pub const SYSTIMER: *const u32 = SYSTIMER_BASE as *const u32;

//==================================================
/// System Timer Control/Status
pub const CS: isize = 0;

/// System Timer Match 0 bit
/// 0 = No match
/// 1 = Match detected
pub const M0: u32 = 0;

/// System Timer Match 1 bit
/// 0 = No match
/// 1 = Match detected
pub const M1: u32 = 1;

/// System Timer Match 2 bit
/// 0 = No match
/// 1 = Match detected
pub const M2: u32 = 2;

/// System Timer Match 3 bit
/// 0 = No match
/// 1 = Match detected
pub const M3: u32 = 3;

//==================================================

/// System Timer Counter Lower 32 bits
pub const CLO: isize = 1;

/// System Timer Counter Higher 32 bits
pub const CHI: isize = 2;

/// System Timer Compare 0
pub const C0: isize = 3;

/// System Timer Compare 1
pub const C1: isize = 4;

/// System Timer Compare 2
pub const C2: isize = 5;

/// System Timer Compare 3
pub const C3: isize = 6;

#[no_mangle]
pub fn timer_lower_bits() -> u32 {
    unsafe { *(SYSTIMER.offset(CLO)) }
}

#[no_mangle]
pub fn timer_higher_bits() -> u32 {
    unsafe { *(SYSTIMER.offset(CHI)) }
}

/*
#[no_mangle]
pub fn timer_64_bits() -> u64 {
    ((timer_higher_bits() as u64) << 32) + (timer_lower_bits() as u64)
}
*/

#[no_mangle]
pub fn delay_micros(us: u32) {
    let start_time = timer_lower_bits();

    while timer_lower_bits() - start_time < us {
        base::nothing();
    }
}
