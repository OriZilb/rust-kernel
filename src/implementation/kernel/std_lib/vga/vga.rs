mod unsafe_wrappers;

// VGA buffer constants and color enum
use super::consts::*;
use super::utils::*;
static mut CURSOR: Cursor = Cursor::Position { col: 0, row: 0 };

/// Scroll the screen up by one line
/// This function moves all lines up by one and clears the last line
/// Note that this last line isn't saved anywhere currently, and hence after the line is deleted
/// it is lost forever.
///
/// # Safety
/// This function performs raw pointer arithmetic and writes directly to the VGA buffer.
/// It should be used with caution to avoid undefined behavior.
///
/// #
pub fn scroll_up() {
    unsafe {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let from_offset = buffer_offset(col, row);
                let to_offset = buffer_offset(col, row - 1);
                *VGA_BUFFER_ADDRESS.add(to_offset) = *VGA_BUFFER_ADDRESS.add(from_offset);
                *VGA_BUFFER_ADDRESS.add(to_offset + 1) = *VGA_BUFFER_ADDRESS.add(from_offset + 1);
            }
        }

        // Clear the last line
        let last_row = BUFFER_HEIGHT - 1;
        for col in 0..BUFFER_WIDTH {
            let offset = buffer_offset(col, last_row);
            *VGA_BUFFER_ADDRESS.add(offset) = b' ';
            *VGA_BUFFER_ADDRESS.add(offset + 1) = foreground_background_colors(ColorCodeVga::White, ColorCodeVga::Black);
        }
    }
}

// Print a single character with color at the current cursor
pub fn print_char_with_color(byte: u8, foreground_color: ColorCodeVga, background_color: ColorCodeVga) {
    unsafe {
        let mut col;
        let mut row;

        match CURSOR {
            Cursor::Position { col: c, row: r } => { col = c; row = r; }
        }

        let offset = buffer_offset(col, row);

        match byte {
            b'\n' => {
                col = 0;
                row += 1;
            }
            b'\x08' => { // Backspace
                if col > 0 {
                    col -= 1;
                    *VGA_BUFFER_ADDRESS.add(offset) = b' ';
                } else if row > 0 {
                    *VGA_BUFFER_ADDRESS.add(buffer_offset(col, row)) = b' ';
                    row -= 1;
                    col = BUFFER_WIDTH - 1;
                } else if row == 0 {
                    *VGA_BUFFER_ADDRESS.add(buffer_offset(col, row)) = b' ';
                }
            }
            _ => {
                *VGA_BUFFER_ADDRESS.add(offset) = byte;
                *VGA_BUFFER_ADDRESS.add(offset + 1) = foreground_background_colors(foreground_color, background_color);
                col += 1;

                if col >= BUFFER_WIDTH {
                    col = 0;
                    row += 1;
                }
            }
        }

        if row >= BUFFER_HEIGHT {
            scroll_up();
            row = BUFFER_HEIGHT - 1;
        }

        CURSOR = Cursor::Position { col, row };
    }
}

// Print a single character with default colors
pub fn print_char(byte: u8) {
    print_char_with_color(byte, ColorCodeVga::White, ColorCodeVga::Black);
}

// Print a string with color
pub fn print_string_with_color(to_print: &str, foreground_color: ColorCodeVga, background_color: ColorCodeVga) {
    for byte in to_print.bytes() {
        print_char_with_color(byte, foreground_color, background_color);
    }
}

// Print a string with default colors
pub fn print_string(to_print: &str) {
    print_string_with_color(to_print, ColorCodeVga::White, ColorCodeVga::Black);
}

// Print a line with default colors
pub fn println(to_print: &str) {
    print_string(to_print);
    print_char_with_color(b'\n', ColorCodeVga::White, ColorCodeVga::Black);
}

// Print a line with color
pub fn println_with_color(to_print: &str, foreground_color: ColorCodeVga, background_color: ColorCodeVga)
{
    print_string_with_color(to_print, foreground_color, background_color);
    print_char_with_color(b'\n', foreground_color, background_color);
}

// Fill the entire screen with a color
pub fn fill_screen(color: ColorCodeVga) {
    for i in 0..(BUFFER_WIDTH * BUFFER_HEIGHT) {
        let offset = i * 2;
        unsafe {
            *VGA_BUFFER_ADDRESS.add(offset) = b' ';
            *VGA_BUFFER_ADDRESS.add(offset + 1) = color as u8;
        }
    }
}

// Clear the screen to black
pub fn clear_screen() {
    fill_screen(ColorCodeVga::Black);
}
