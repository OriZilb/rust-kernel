use pc_keyboard::{Keyboard, layouts::Us104Key, ScancodeSet1, HandleControl};
use crate::arch::drivers::keyboard::read_keyboard_input;
use crate::interrupts::irq_handler_types::IrqResult;
use crate::arch::CpuState;
use crate::std_lib::vga::*;
use lazy_static::lazy_static;
use spin::{Mutex, MutexGuard};

lazy_static! {
    static ref KEYBOARD: Mutex<Keyboard<Us104Key, ScancodeSet1>> =
        Mutex::new(Keyboard::new(ScancodeSet1::default(), Us104Key, HandleControl::Ignore));
}
pub(super) fn keyboard_interrupt_handler(_stack_frame: &CpuState) -> IrqResult {
    let keyboard_input = unsafe{read_keyboard_input()};
    match keyboard_input {
        None => {
            IrqResult::NotHandled
        },
        Some(scancode) => {
            let mut keyboard = KEYBOARD.lock();
            if let Ok(Some(_event)) = keyboard.add_byte(scancode) {
                if let Some(key) = keyboard.process_keyevent(_event) {
                    match key {
                        pc_keyboard::DecodedKey::Unicode(character) => print_byte(character as u8),
                        pc_keyboard::DecodedKey::RawKey(_key) => {}
                    }
                }
            }
            IrqResult::Handled
        }
    }
}