// kernel.rs
// http://blog.thiago.me/raspberry-pi-bare-metal-programming-with-rust/

#![feature(lang_items, asm)]
#![crate_type = "staticlib"]
#![no_std]

extern crate rusty_metal_raspberry_pi;

//use rusty_metal_raspberry_pi::base;
use rusty_metal_raspberry_pi::gpio;
use rusty_metal_raspberry_pi::systimer;

//mod base;
//mod gpio;
//mod systimer;

#[no_mangle]
pub extern fn main() {  

    for _ in 0..4 {
        gpio::digital_write(gpio::ACT_LED, true);
        systimer::delay_micros(100000);

        gpio::digital_write(gpio::ACT_LED, false);
        systimer::delay_micros(100000);
    }

    loop {
        gpio::digital_write(gpio::ACT_LED, true);
        systimer::delay_micros(1000000);
        
        gpio::digital_write(gpio::ACT_LED, false);
        systimer::delay_micros(500000);
    }
}

// The Rust compiler expects these because there is no stdlib
//#[lang = "eh_personality"] extern fn eh_personality() {}
//#[lang = "panic_fmt"] extern fn panic_fmt() {}

// Satisfies the linker's need for _exit, _kill, etc
//#[no_mangle]
//pub extern fn __aeabi_unwind_cpp_pr0 () {}
