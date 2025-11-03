/*
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use x86_64::instructions::port::Port;
use pc_keyboard::{Keyboard, layouts::Us104Key, ScancodeSet1, HandleControl};
use crate::std_lib::vga;
use super::consts::*;


pub(super) extern "x86-interrupt" fn debug_handler(_stack_frame: InterruptStackFrame) {
    vga::println("EXCEPTION: DEBUG");
    loop { x86_64::instructions::hlt(); }
}

pub(super) extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    pic_end_of_interrupt(IrqNumber::SystemTimer as u8);
}

pub(super) extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    use x86_64::instructions::interrupts::without_interrupts;

    unsafe {
        without_interrupts(|| {
            if let Some(ref mut keyboard) = KEYBOARD.lock().as_mut() {
                let mut port = Port::new(0x60);
                let scancode: u8 = port.read();
                if let Ok(Some(_event)) = keyboard.add_byte(scancode) {
                    if let Some(key) = keyboard.process_keyevent(_event) {
                        match key {
                            pc_keyboard::DecodedKey::Unicode(character) => vga::print_char(character as u8),
                            pc_keyboard::DecodedKey::RawKey(_key) => {} 
                        }
                    }
                }
            }
            pic_end_of_interrupt(IrqNumber::Keyboard as u8);
        });
    }
}


fn pic_end_of_interrupt(irq: u8) {
    let mut master = Port::new(MASTER_CONTROL);
    let mut slave = Port::new(SLAVE_CONTROL);
    const END_OF_INTERRUPT: u8 = 0x20;

    if irq >= 8 { unsafe { slave.write(END_OF_INTERRUPT) }; }
    unsafe { master.write(END_OF_INTERRUPT) };
}

 */