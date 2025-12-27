use crate::std_lib::vga::consts::{Position, BUFFER_HEIGHT, BUFFER_WIDTH};
use crate::std_lib::vga::writer::writer::Writer;
use crate::std_lib::vga::{unsafe_wrappers, ColorCodeVga, VGAChar};

impl Writer {
    /// Print a string using the Writer's current foreground and background colors.
    /// # Arguments
    /// * `self` - The Writer instance
    /// * `string` - The string slice to print
    /// # Returns
    /// * `()` - No return value
    pub(super) fn print(&mut self, string: &str) {
        for byte in string.bytes() {
            self.handle_char(byte);
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
    pub(super) fn print_colored(
        &mut self,
        string: &str,
        foreground: ColorCodeVga,
        background: ColorCodeVga,
    ) {
        let old_foreground = self.get_foreground_color();
        let old_background = self.get_background_color();
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
    pub(super) fn println(&mut self, string: &str) {
        self.print(string);
        self.handle_char(b'\n');
    }

    /// Print a string with specified foreground and background colors, followed by a newline.
    /// # Arguments
    /// * `self` - The Writer instance
    /// * `string` - The string slice to print
    /// * `foreground` - The foreground color to use
    /// * `background` - The background color to use
    /// # Returns
    /// * `()` - No return value
    pub(super) fn println_colored(
        &mut self,
        string: &str,
        foreground: ColorCodeVga,
        background: ColorCodeVga,
    ) {
        self.print_colored(string, foreground, background);
        self.handle_char(b'\n');
    }

    /// Fills the entire screen with the specified color.
    /// # Arguments
    /// * `self` - The Writer instance
    /// * `color` - The color to fill the screen with
    /// # Returns
    /// * `()` - No return value
    pub(super) fn fill_screen(&mut self, color: ColorCodeVga) {
        for row in 0..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let position = Position { col, row };
                let blank_char =
                    VGAChar::new_from_ascii_foreground_background_colors(b' ', color, color);
                unsafe_wrappers::write_char(position, blank_char).unwrap();
            }
        }
        self.set_cursor_position(Position { col: 0, row: 0 });
    }

    /// Clears the screen by filling it with black color.
    /// # Arguments
    /// * `self` - The Writer instance
    /// # Returns
    /// * `()` - No return value
    pub(super) fn clear_screen(&mut self) {
        self.fill_screen(ColorCodeVga::Black);
    }
}
