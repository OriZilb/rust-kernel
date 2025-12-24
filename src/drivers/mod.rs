mod keyboard;
use crate::arch::initialize_interrupts;
use crate::interrupts::register_driver;
use crate::std_lib::vga::print;
pub fn load_drivers() {
    unsafe {initialize_interrupts()};
    register_driver(1, keyboard::keyboard_interrupt_handler, "keyboard".parse().unwrap());
}