#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![allow(dead_code)]
#![allow(unused_imports)]

mod panic;
mod interrupts;
mod arch;
mod std_lib;
use std_lib::vga::{print, clear_screen};
mod drivers;
use drivers::load_drivers;

#[no_mangle] // prevents Rust from mangling the name
pub extern "C" fn kernel_main() -> ! {
    // Initialize VGA writer
    clear_screen();
    print("Hello 123");
    load_drivers();
    loop {
        x86_64::instructions::hlt();
    }
}
