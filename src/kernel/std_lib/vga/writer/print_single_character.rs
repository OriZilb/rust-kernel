use super::writer::Writer;
use crate::std_lib::vga::consts::{Position, BUFFER_HEIGHT, BUFFER_WIDTH};
use crate::std_lib::vga::{unsafe_wrappers, ColorCodeVga, VGAChar};

const ASCII_PRINTABLE_LOWER_BOUNDARY: u8 = 32;
const ASCII_PRINTABLE_UPPER_BOUNDARY: u8 = 126;
const SPACES_IN_TAB: usize = 4;
impl Writer {
    /// Handle a single character input.
    /// This function processes special characters like newline and carriage return.
    /// # Arguments
    /// * `self` - The Writer instance
    /// * `char` - The ASCII character to handle
    /// # Returns
    /// * `()` - No return value
    pub(super) fn handle_char(&mut self, char: u8) {
        match char {
            ASCII_PRINTABLE_LOWER_BOUNDARY..=ASCII_PRINTABLE_UPPER_BOUNDARY => {
                self.print_char(char);
            }
            b'\n' => {
                self.handle_newline();
            }
            b'\r' => {
                self.handle_carriage_return();
            }
            b'\x08' => {
                // Backspace
                self.handle_backspace();
            }
            b'\t' => {
                // Tab
                for _ in 0..SPACES_IN_TAB {
                    self.print_char(b' ');
                }
            }
            _ => {}
        }
    }

    /// Print a single character with the Writer's current foreground and background colors.
    /// # Arguments
    /// * `self` - The Writer instance
    /// * `char` - The ASCII character to print
    /// # Returns
    /// * `()` - No return value
    fn print_char(&mut self, char: u8) {
        unsafe_wrappers::write_char(
            self.get_cursor_position(),
            VGAChar::new_from_ascii_foreground_background_colors(
                char,
                self.get_foreground_color(),
                self.get_background_color(),
            ),
        )
        .unwrap();

        // move the cursor forward
        if self.get_cursor_position().col == BUFFER_WIDTH - 1 {
            self.handle_newline();
        } else {
            let mut current_position = self.get_cursor_position();
            current_position.col += 1;
            self.set_cursor_position(current_position);
        }
    }

    /// Handle a newline character.
    /// This function moves the cursor to the beginning of the next line, scrolling the screen if necessary.
    /// # Arguments
    /// * `self` - The Writer instance
    /// # Returns
    /// * `()` - No return value
    fn handle_newline(&mut self) {
        let mut current_position = self.get_cursor_position();
        if current_position.row == BUFFER_HEIGHT - 1 {
            Writer::scroll_up();
        } else {
            current_position.row += 1;
        }
        current_position.col = 0;
        self.set_cursor_position(current_position);
    }

    /// Handle a carriage return character.
    /// This function moves the cursor to the beginning of the current line.
    /// # Arguments
    /// * `self` - The Writer instance
    /// # Returns
    /// * `()` - No return value
    fn handle_carriage_return(&mut self) {
        let mut current_position = self.get_cursor_position();
        current_position.col = 0;
        self.set_cursor_position(current_position);
    }

    /// Handle a backspace character.
    /// This function moves the cursor back by one position, erasing the character at that position.
    /// # Arguments
    /// * `self` - The Writer instance
    /// # Returns
    /// * `()` - No return value
    fn handle_backspace(&mut self) {
        let mut current_position = self.get_cursor_position();
        if current_position.col > 0 {
            current_position.col -= 1;
        } else if current_position.row > 0 {
            current_position.row -= 1;
            current_position.col = BUFFER_WIDTH - 1;
        }
        self.set_cursor_position(current_position);
        self.print_char(b' ');
    }

    /// Scroll the screen up by one line
    /// This function moves all lines up by one and clears the last line
    /// Note that this last line isn't saved anywhere currently, and hence after the line is deleted
    /// it is lost forever.
    ///
    /// # Safety
    /// This function uses the unsafe wrappers to manipulate the VGA buffer in a safe manner.
    fn scroll_up() {
        // Move each line up by one
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                unsafe_wrappers::copy_char(Position { col, row }, Position { col, row: row - 1 })
                    .unwrap();
            }
        }

        // Clear the last line
        let blank = VGAChar::new_from_ascii_foreground_background_colors(
            b' ',
            ColorCodeVga::White,
            ColorCodeVga::Black,
        );
        let last_row = BUFFER_HEIGHT - 1;
        for col in 0..BUFFER_WIDTH {
            unsafe_wrappers::write_char(Position { col, row: last_row }, blank).unwrap();
        }
    }
}
