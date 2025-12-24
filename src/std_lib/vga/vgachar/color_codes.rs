// Color codes for VGA vgachar mode. Note that each color is represented by 4 bits.
#[derive(Clone, Copy, PartialEq, Eq)]
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

impl ColorCodeVga {
    /// Converts a u8 value to a ColorCodeVga enum variant
    /// Returns None if the value is out of range (0-15)
    pub fn from_u8(value: u8) -> Option<ColorCodeVga> {
        match value {
            0x0 => Some(ColorCodeVga::Black),
            0x1 => Some(ColorCodeVga::Blue),
            0x2 => Some(ColorCodeVga::Green),
            0x3 => Some(ColorCodeVga::Cyan),
            0x4 => Some(ColorCodeVga::Red),
            0x5 => Some(ColorCodeVga::Magenta),
            0x6 => Some(ColorCodeVga::Brown),
            0x7 => Some(ColorCodeVga::LightGray),
            0x8 => Some(ColorCodeVga::DarkGray),
            0x9 => Some(ColorCodeVga::LightBlue),
            0xa => Some(ColorCodeVga::LightGreen),
            0xb => Some(ColorCodeVga::LightCyan),
            0xc => Some(ColorCodeVga::LightRed),
            0xd => Some(ColorCodeVga::Pink),
            0xe => Some(ColorCodeVga::Yellow),
            0xf => Some(ColorCodeVga::White),
            _ => None,
        }
    }
}
