use crate::std_lib::vga::consts::Position;
use crate::std_lib::vga::ColorCodeVga;

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
    pub(super) fn set_cursor_position(&mut self, position: Position) {
        self.cursor_position = position;
    }

    /// Sets the foreground color of the Writer.
    /// # Arguments
    /// * `self` - The Writer instance
    /// * `foreground` - The new foreground color
    /// # Returns
    /// * `()` - No return value
    pub(super) fn set_foreground_color(&mut self, foreground: ColorCodeVga) {
        self.foreground_color = foreground;
    }

    /// Sets the background color of the Writer.
    /// # Arguments
    /// * `self` - The Writer instance
    /// * `foreground` - The new foreground color
    /// # Returns
    /// * `()` - No return value
    pub(super) fn set_background_color(&mut self, background: ColorCodeVga) {
        self.background_color = background;
    }

    /// Gets the current cursor position of the Writer.
    /// # Arguments
    /// * `self` - The Writer instance
    /// # Returns
    /// * `Position` - The current cursor position
    pub(super) fn get_cursor_position(&self) -> Position {
        self.cursor_position
    }

    /// Gets the current foreground color of the Writer.
    /// # Arguments
    /// * `self` - The Writer instance
    /// # Returns
    /// * `ColorCodeVga` - The current foreground color
    pub(super) fn get_foreground_color(&self) -> ColorCodeVga {
        self.foreground_color
    }

    /// Gets the current background color of the Writer.
    /// # Arguments
    /// * `self` - The Writer instance
    /// # Returns
    /// * `ColorCodeVga` - The current background color
    pub(super) fn get_background_color(&self) -> ColorCodeVga {
        self.background_color
    }
}
