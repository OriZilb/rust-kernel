use super::color_codes::ColorCodeVga;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct VGAChar {
    pub char: u16,
}

impl VGAChar {
    
    /// Create a new vgachar from a u16 value
    /// # Arguments
    /// * `char` - u16 value combining ASCII character and color code
    /// # Returns
    /// * `vgachar` - The constructed vgachar
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
    
    /// Create a vgachar from ASCII character and separate foreground and background colors
    /// # Arguments
    /// * `ascii_character` - The ASCII character (0-255)
    /// * `foreground` - Foreground color code (0-15)
    /// * `background` - Background color code (0-15)
    /// # Returns
    /// * `vgachar` - The constructed vgachar
    pub fn new_from_ascii_foreground_background_colors(ascii_character: u8, foreground: ColorCodeVga, background: ColorCodeVga) -> Self {
        let color_code = Self::combine_foreground_background_colors(foreground, background);
        VGAChar::new_from_ascii_and_color(ascii_character, color_code)
    }

    /// Create a vgachar from the color byte and ASCII character
    /// # Arguments
    /// * `ascii_character` - The ASCII character (0-255)
    /// * `color` - Combined color code byte (0-255)
    /// # Returns
    /// * `vgachar` - The constructed vgachar
    pub fn new_from_ascii_and_color(ascii_character: u8, color: u8) -> Self {
        VGAChar {
            char: (color as u16) << 8 | (ascii_character as u16),
        }
    }
    
    /// Gets the ascii character represented in the vgachar struct
    /// # Arguments
    /// * `self` - The vgachar object to get its ascii character from
    /// # Returns
    /// *`u8` - The ascii representation of the char
    pub fn get_ascii(&self) -> u8 {
        (self.char & 0x00FF) as u8
    }
    
    /// Gets the color byte from the vgachar
    /// The lower byte represents the ASCII character, and the upper byte represents the color code
    /// # Returns
    /// * `u16` - The combined u16 value
    pub fn get_color(&self) -> u8 {
        (self.char >> 8) as u8
    }

    /// Get the foreground color from a vgachar
    /// # Arguments
    /// * `self` - The vgachar to get the foreground color of
    /// # Returns
    /// * `ColorCodeVga` - the foreground color of the char
    pub fn get_foreground_color(&self) -> ColorCodeVga {
        let color = self.get_color();
        let foreground_color = color & 0x0F;
        match ColorCodeVga::from_u8(foreground_color) {
            Some(color) => color,
            _ => ColorCodeVga::Black // will never happen
        }
    }

    /// Get the background color from a vgachar
    /// # Arguments
    /// * `self` - The vgachar to get the background color of
    /// # Returns
    /// * `ColorCodeVga` - the background color of the char
    pub fn get_background_color(&self) -> ColorCodeVga {
        let background_color = (self.char >> 12) as u8;
        match ColorCodeVga::from_u8(background_color) {
            Some(color) => color,
            _ => ColorCodeVga::Black // will never happen
        }
    }
}