use x86_64::instructions::port::Port;
const KEYBOARD_DATA_PORT: u16 = 0x60;
const KEYBOARD_STATUS_PORT: u16 = 0x64;
const KEYBOARD_READY_BIT: u8 = 0x01;
const KEYBOARD_MOUSE_DISTINCTION_BIT: u8 = 0x20;
pub unsafe fn read_keyboard_input() -> Option<u8> {
    let keyboard_status: u8 = Port::new(KEYBOARD_STATUS_PORT).read();
    let keyboard_input: u8 = Port::new(KEYBOARD_DATA_PORT).read();
    if keyboard_status & KEYBOARD_READY_BIT != 0
        && keyboard_status & KEYBOARD_MOUSE_DISTINCTION_BIT == 0
    {
        Some(keyboard_input)
    } else {
        None
    }
}
