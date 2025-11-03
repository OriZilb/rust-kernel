#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

mod panic;
mod std_lib;

use std_lib::vga::*;
#[no_mangle] // prevents Rust from mangling the name
pub extern "C" fn kernel_main() -> ! {
    // Initialize VGA writer
    clear_screen();
    print("Hello 123");

    loop {
        x86_64::instructions::hlt();
    }
}
