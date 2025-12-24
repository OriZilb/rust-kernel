use super::writer::Writer;
use crate::std_lib::vga::consts::Position;
use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    /// A global Writer instance protected by a Mutex for synchronized access.
    /// This ensures that only one thread can write to the VGA buffer at a time.
    /// This is crucial for maintaining the integrity of the output on the screen.
    ///
    /// This file includes all the public printing functions Writer provides, but wraps them
    /// to use the global Writer instance.
    pub static ref GLOBAL_WRITER: Mutex<Writer> = Mutex::new(Writer::default());
}
pub fn print(string: &str) {
    GLOBAL_WRITER.lock().print(string);
}

pub fn print_colored(
    string: &str,
    foreground: crate::std_lib::vga::ColorCodeVga,
    background: crate::std_lib::vga::ColorCodeVga,
) {
    GLOBAL_WRITER
        .lock()
        .print_colored(string, foreground, background);
}

pub fn println(string: &str) {
    GLOBAL_WRITER.lock().println(string);
}

pub fn println_colored(
    string: &str,
    foreground: crate::std_lib::vga::ColorCodeVga,
    background: crate::std_lib::vga::ColorCodeVga,
) {
    GLOBAL_WRITER
        .lock()
        .println_colored(string, foreground, background);
}

pub fn fill_screen(color: crate::std_lib::vga::ColorCodeVga) {
    GLOBAL_WRITER.lock().fill_screen(color);
}
pub fn clear_screen() {
    GLOBAL_WRITER.lock().clear_screen();
}

pub fn get_cursor_position() -> Position {
    GLOBAL_WRITER.lock().get_cursor_position()
}

pub fn set_cursor_position(position: Position) {
    GLOBAL_WRITER.lock().set_cursor_position(position);
}

pub fn get_foreground_color() -> crate::std_lib::vga::ColorCodeVga {
    GLOBAL_WRITER.lock().get_foreground_color()
}

pub fn set_foreground_color(foreground: crate::std_lib::vga::ColorCodeVga) {
    GLOBAL_WRITER.lock().set_foreground_color(foreground);
}

pub fn get_background_color() -> crate::std_lib::vga::ColorCodeVga {
    GLOBAL_WRITER.lock().get_background_color()
}

pub fn set_background_color(background: crate::std_lib::vga::ColorCodeVga) {
    GLOBAL_WRITER.lock().set_background_color(background);
}