//! gpio
//!
//! Module for using the GPIO pins

#![allow(dead_code)]

use base;

//pub const GPIO_BASE: u32 = 0x20200000;
pub const GPIO_BASE: u32 = base::PERIPHERAL_BASE + 0x200000;

pub const GPIO: *const u32 = GPIO_BASE as *const u32;


pub const GPSEL0: isize = 0;
pub const GPSEL1: isize = 1;
pub const GPSEL2: isize = 2;
pub const GPSEL3: isize = 3;
pub const GPSEL4: isize = 4;
pub const GPSEL5: isize = 5;

pub const GPSET0: isize = 7;
pub const GPSET1: isize = 8;

pub const GPCLR0: isize = 10;
pub const GPCLR1: isize = 11;

pub const GPLEV0: isize = 13;
pub const GPLEV1: isize = 14;

pub const GPEDS0: isize = 16;
pub const GPEDS1: isize = 17;

pub const GPREN0: isize = 19;
pub const GPREN1: isize = 20;

pub const GPFEN0: isize = 22;
pub const GPFEN1: isize = 23;

pub const GPPUD: isize = 25;
pub const GPPUDCLK0: isize = 26;
pub const GPPUDCLK1: isize = 27;

/// Pin for the onboard led on the pi zero
/// Note that it is inverted on the pi zero
pub const ACT_LED: u32 = 47;

pub enum Modes {
    In = 0b000,
    Out = 0b001,
    Alt0 = 0b100,
    Alt1 = 0b101,
    Alt2 = 0b110,
    Alt3 = 0b111,
    Alt4 = 0b011,
    Alt5 = 0b010,
}

pub fn get_gpsel(pin: u32) -> Option<isize> {
    match pin {
        0...9 => Some(GPSEL0),
        10...19 => Some(GPSEL1),
        20...29 => Some(GPSEL2),
        30...39 => Some(GPSEL3),
        40...49 => Some(GPSEL4),
        50...53 => Some(GPSEL5),
        _ => None,
    }
}

/// Set a pin to a mode specified in gpio::Modes
///
/// If pin does not exist, it will do nothing
pub fn set_mode(pin: u32, mode: Modes) {
    unsafe {
        let byte = GPIO.offset(match get_gpsel(pin) {
            Some(offset) => offset,
            None => return,
        }) as *mut u32;

        let shift = (pin % 10) * 3;

        let mut reg = *(byte);

        // Clear the 3 bits needed to set to output
        reg &= !(0b111 << shift);

        // Set the 3 bits needed
        reg |= (mode as u32) << shift;

        *(byte) = reg;
    }
}

/// Write to a digital pin
///
/// pin: The pin to write, 0-53
/// state: The state to write. true -> high, false -> low
#[no_mangle]
pub fn digital_write(pin: u32, state: bool) {

    // Get the location in memory that controls the bank of
    // pins with this pin in it. There are seoerate banks
    // for pins 1-32 and 33-54, as well as for set and reset
    let byte = if pin < 32 {
        match state {
            true => unsafe { GPIO.offset(GPSET0) as *mut u32 },
            false => unsafe { GPIO.offset(GPCLR0) as *mut u32 },
        }
    } else {
        match state {
            true => unsafe { GPIO.offset(GPSET1) as *mut u32 },
            false => unsafe { GPIO.offset(GPCLR1) as *mut u32 },
        }
    };

    // Get the bit for this pin in the memory location
    let bit = if pin < 32 { pin } else { pin - 32 };

    // Set the bit
    unsafe { *(byte) = 1 << bit }
}


pub enum PullModes {
    Off = 0b00,
    Down = 0b01,
    Up = 0b10,
}

pub fn set_pull_mode(pin: u32, mode: PullModes) {

    if pin > 53 {
        return;
    }

    let bit = if pin < 32 { pin } else { pin - 32 };

    let clk_byte = if pin < 32 {
        unsafe { base::get_reg(GPIO, GPPUDCLK0) }
    } else {
        unsafe { base::get_reg(GPIO, GPPUDCLK1) }
    };

    unsafe {

        *(base::get_reg(GPIO, GPPUD)) = mode as u32;

        for _ in 0..150 {
            base::nothing();
        }

        *(clk_byte) = 1 << bit;

        for _ in 0..150 {
            base::nothing();
        }

        *(base::get_reg(GPIO, GPPUD)) = PullModes::Off as u32;

        *(clk_byte) = 0;
    }
}
