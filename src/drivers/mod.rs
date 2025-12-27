mod keyboard;
use crate::arch::interrupts::initialize_interrupts;
use crate::interrupts::consts::IrqNumber;
use crate::interrupts::register_driver;
use crate::std_lib::vga::print;
pub fn load_drivers() {
    unsafe { initialize_interrupts() };
    register_driver(
        IrqNumber::Keyboard as usize,
        keyboard::keyboard_interrupt_handler,
        "keyboard".parse().unwrap(),
    );
}
