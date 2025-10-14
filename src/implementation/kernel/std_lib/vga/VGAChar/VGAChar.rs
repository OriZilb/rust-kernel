use crate::std_lib::vga::consts::ColorCodeVga;
pub const BYTES_PER_CHAR: usize = 2; // Each VGA character is represented by 2 bytes
pub struct VGAChar {
    pub char: u16,
}

impl VGAChar {
    pub fn new(char: u16) -> Self {
        VGAChar {
            char,
        }
    }

    /// Combines foreground and background color codes into a single byte
    /// Foreground color occupies the lower 4 bits, background color occupies the upper 4 bits
    ///
    /// # Arguments
    /// * `foreground` - Foreground color code (0-15)
    /// * `background` - Background color code (0-15)
    ///
    /// # Returns
    /// * `u8` - Combined color code byte
    pub fn combine_foreground_background_colors(foreground: ColorCodeVga, background: ColorCodeVga) -> u8 {
        (foreground as u8) | ((background as u8) << 4)
    }
    
    /// Create a VGAChar from ASCII character and separate foreground and background colors
    /// # Arguments
    /// * `ascii_character` - The ASCII character (0-255)
    /// * `foreground` - Foreground color code (0-15)
    /// * `background` - Background color code (0-15)
    /// # Returns
    /// * `VGAChar` - The constructed VGAChar
    pub fn new_from_ascii_foreground_background_colors(ascii_character: u8, foreground: ColorCodeVga, background: ColorCodeVga) -> Self {
        let color_code = Self::combine_foreground_background_colors(foreground, background);
        VGAChar::new_from_ascii_and_color(ascii_character, color_code)
    }

    /// Create a VGAChar from the color byte and ASCII character
    /// # Arguments
    /// * `ascii_character` - The ASCII character (0-255)
    /// * `color` - Combined color code byte (0-255)
    /// # Returns
    /// * `VGAChar` - The constructed VGAChar
    pub fn new_from_ascii_and_color(ascii_character: u8, color: u8) -> Self {
        VGAChar {
            char: (color as u16) << 8 | (ascii_character as u16),
        }
    }
    
    
    pub fn to_ascii(&self) -> u8 {
        (self.char & 0x00FF) as u8
    }
    
    /// Converts the VGAChar to a u16 value
    /// The lower byte represents the ASCII character, and the upper byte represents the color code
    /// # Returns
    /// * `u16` - The combined u16 value
    pub fn to_u16(&self) -> u16 {
        (self.color_code as u16) << 8 | (self.ascii_character as u16)
    }
}