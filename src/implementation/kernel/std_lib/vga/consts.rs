
/** VGA constants and types */
pub const VGA_BUFFER_ADDRESS: *mut u8 = 0xb8000 as *mut u8; // VGA text buffer address

// VGA buffer dimensions
pub const BUFFER_WIDTH: usize = 80;
pub const BUFFER_HEIGHT: usize = 25;

// Color codes for VGA text mode. Note that each color is represented by 4 bits.
#[derive(Clone, Copy)]
pub enum ColorCodeVga {
    Black = 0x0,
    Blue = 0x1,
    Green = 0x2,
    Cyan = 0x3,
    Red = 0x4,
    Magenta = 0x5,
    Brown = 0x6,
    LightGray = 0x7,
    DarkGray = 0x8,
    LightBlue = 0x9,
    LightGreen = 0xa,
    LightCyan = 0xb,
    LightRed = 0xc,
    Pink = 0xd,
    Yellow = 0xe,
    White = 0xf,
}

// Cursor position representation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cursor {
    Position { col: usize, row: usize },
}