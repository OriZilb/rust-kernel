use crate::std_lib::vga::consts::Position;
use crate::std_lib::vga::{unsafe_wrappers, ColorCodeVga, VGAChar};
use crate::std_lib::vga::consts::{BUFFER_HEIGHT, BUFFER_WIDTH};

/// A writer head struct, to keep the state of the vga screen.
/// Generally there should be only one of these, and I'll use a Mutex to enforce that.
/// It keeps track of the cursor position, foreground and background colors.
#[derive(Clone, Copy)]
pub struct Writer {
    cursor_position: Position,
    foreground_color: ColorCodeVga,
    background_color: ColorCodeVga,
}

impl Default for Writer {
    /// Creates a default Writer with cursor at (0,0) and white on black colors.
    fn default() -> Self {
        Writer {
            cursor_position: Position { col: 0, row: 0 },
            foreground_color: ColorCodeVga::White,
            background_color: ColorCodeVga::Black,
        }
    }
}

impl Writer {

    /// Sets the cursor position of the Writer.
    /// # Arguments
    /// * `self` - The Writer instance
    /// * `position` - The new cursor position
    /// # Returns
    /// * `()` - No return value
    pub fn set_cursor_position(&mut self, position: Position) {
        self.cursor_position = position;
    }

    /// Sets the foreground color of the Writer.
    /// # Arguments
    /// * `self` - The Writer instance
    /// * `foreground` - The new foreground color
    /// # Returns
    /// * `()` - No return value
    pub fn set_foreground_color(&mut self, foreground: ColorCodeVga) {
        self.foreground_color = foreground;
    }

    /// Sets the background color of the Writer.
    /// # Arguments
    /// * `self` - The Writer instance
    /// * `foreground` - The new foreground color
    /// # Returns
    /// * `()` - No return value
    pub fn set_background_color(&mut self, background: ColorCodeVga) {
        self.background_color = background;
    }

    /// Print a single VGAChar at the current cursor position. Note that this way we disregard the
    /// Writer's foreground and background colors, as the VGAChar already contains color information.
    /// # Arguments
    /// * `self` - The Writer instance
    /// * `character` - The VGAChar to print
    /// # Returns
    /// * `()` - No return value
    fn print_vgachar(&self, character: VGAChar) {
        
        match character.get_ascii() {
            
        }
        unsafe_wrappers::write_char(self.cursor_position, character).unwrap();
    }

    /// Print a single character with the Writer's current foreground and background colors.
    /// # Arguments
    /// * `self` - The Writer instance
    /// * `char` - The ASCII character to print
    /// # Returns
    /// * `()` - No return value
    pub fn print_char(&self, char: u8) {
        self.print_vgachar(VGAChar::new_from_ascii_foreground_background_colors(
            char,
            self.foreground_color,
            self.background_color,
        ));
    }

    /// Print a string using the Writer's current foreground and background colors.
    /// # Arguments
    /// * `self` - The Writer instance
    /// * `string` - The string slice to print
    /// # Returns
    /// * `()` - No return value
    pub fn print(&self, string: &str) {
        for byte in string.bytes() {
            self.print_char(byte);
        }
    }

    /// Print a string with specified foreground and background colors.
    /// # Arguments
    /// * `self` - The Writer instance
    /// * `string` - The string slice to print
    /// * `foreground` - The foreground color to use
    /// * `background` - The background color to use
    /// # Returns
    /// * `()` - No return value
    pub fn print_colored(&mut self, string: &str, foreground: ColorCodeVga, background: ColorCodeVga) {
        let old_foreground = self.foreground_color;
        let old_background = self.background_color;
        self.set_foreground_color(foreground);
        self.set_background_color(background);
        self.print(string);
        self.set_foreground_color(old_foreground);
        self.set_background_color(old_background);
    }

    /// Print a string followed by a newline using the Writer's current foreground and background colors.
    /// # Arguments
    /// * `self` - The Writer instance
    /// * `string` - The string slice to print
    /// # Returns
    /// * `()` - No return value
    pub fn println(&self, string: &str) {
        self.print(string);
        self.print_char(b'\n');
    }

    /// Print a string with specified foreground and background colors, followed by a newline.
    /// # Arguments
    /// * `self` - The Writer instance
    /// * `string` - The string slice to print
    /// * `foreground` - The foreground color to use
    /// * `background` - The background color to use
    /// # Returns
    /// * `()` - No return value
    pub fn println_colored(&mut self, string: &str, foreground: ColorCodeVga, background: ColorCodeVga) {
        self.print_colored(string, foreground, background);
        self.print_char(b'\n');
    }

    /// Fills the entire screen with the specified color.
    /// # Arguments
    /// * `self` - The Writer instance
    /// * `color` - The color to fill the screen with
    /// # Returns
    /// * `()` - No return value
    pub fn fill_screen(color: ColorCodeVga) {
        for row in 0..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let position = Position { col, row };
                let blank_char = VGAChar::new_from_ascii_foreground_background_colors(
                    b' ',
                    color,
                    color,
                );
                unsafe_wrappers::write_char(position, blank_char).unwrap();
            }
        }
    }

    /// Clears the screen by filling it with black color.
    /// # Arguments
    /// * `self` - The Writer instance
    /// # Returns
    /// * `()` - No return value
    pub fn clear_screen() {
        Writer::fill_screen(ColorCodeVga::Black);
    }

    /// Scroll the screen up by one line
    /// This function moves all lines up by one and clears the last line
    /// Note that this last line isn't saved anywhere currently, and hence after the line is deleted
    /// it is lost forever.
    ///
    /// # Safety
    /// This function uses the unsafe wrappers to manipulate the VGA buffer in a safe manner.
    pub fn scroll_up() {
        // Move each line up by one
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                unsafe_wrappers::copy_char(Position { col, row }, Position { col, row: row - 1 }).unwrap();
            }
        }

        // Clear the last line
        let blank = VGAChar::new_from_ascii_foreground_background_colors(b' ', ColorCodeVga::White, ColorCodeVga:: Black);
        let last_row = BUFFER_HEIGHT - 1;
        for col in 0..BUFFER_WIDTH {
            unsafe_wrappers::write_char(Position {col, row: last_row}, blank).unwrap();
        }
    }

}