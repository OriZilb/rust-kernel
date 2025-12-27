use crate::std_lib::vga::vgachar::VGAChar;

/** VGA constants and types */
pub const VGA_BUFFER_ADDRESS: *mut VGAChar = 0xb8000 as *mut VGAChar; // VGA vgachar buffer address

// VGA buffer dimensions
pub const BUFFER_WIDTH: usize = 80;
pub const BUFFER_HEIGHT: usize = 25;
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Position {
    pub col: usize,
    pub row: usize,
}
