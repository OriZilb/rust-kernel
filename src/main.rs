#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![allow(dead_code)]

mod panic;
mod interrupts;
mod std_lib;

use x86_64::instructions::port::Port;
use x86_64::structures::idt::InterruptStackFrame;
use std_lib::vga::*;
mod arch;
use arch::x86_64::interrupts::initialize_pic::init_pic;
use crate::arch::x86_64::interrupts::idt::init_idt;
use crate::arch::x86_64::interrupts::idt::DISPATCH_TABLE;
use crate::arch::x86_64::interrupts::initialize_pic::PICS;
use crate::interrupts::definitions::IrqResult;

#[no_mangle] // prevents Rust from mangling the name
pub extern "C" fn kernel_main() -> ! {
    // Initialize VGA writer
    clear_screen();
    print("Hello 123");
    unsafe {
        init_pic();
        init_idt();
    };
    let success = DISPATCH_TABLE.lock().register_handler(1, |a: &InterruptStackFrame| {let scancode: u8 = unsafe { Port::new(0x60).read() };print("a"); unsafe{PICS.lock().notify_end_of_interrupt(33)}; IrqResult::Handled} , "Keyboard".parse().unwrap());
    x86_64::instructions::interrupts::enable();
    unsafe{PICS.lock().write_masks(0xfd, 0xff); }
    if success{
        print("Registered keyboard handler\n");
    } else {
        print("Failed to register keyboard handler\n");
    }
    loop {
        x86_64::instructions::hlt();
    }
}
