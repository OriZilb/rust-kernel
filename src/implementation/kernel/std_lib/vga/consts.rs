use crate::std_lib::vga::VGAChar::VGAChar::VGAChar;

/** VGA constants and types */
pub const VGA_BUFFER_ADDRESS: *mut VGAChar = 0xb8000 as *mut VGAChar; // VGA VGAChar buffer address

// VGA buffer dimensions
pub const BUFFER_WIDTH: usize = 80;
pub const BUFFER_HEIGHT: usize = 25;

// Cursor position representation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cursor {
    Position { col: usize, row: usize },
}

pub struct Position {
    pub col: usize,
    pub row: usize,
}