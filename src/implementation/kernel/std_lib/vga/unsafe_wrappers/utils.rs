use super::super::consts::*;

/// Calculate the byte offset in the VGA text buffer for a given column and row.
/// Each character cell in VGA text mode consists of 2 bytes: one for the ASCII character
/// and one for the color attribute (foreground and background colors).
/// # Arguments
/// * `col` - Column index (0 to BUFFER_WIDTH - 1)
/// * `row` - Row index (0 to BUFFER_HEIGHT - 1)
/// # Returns
/// * `Option<usize>` - The byte offset in the VGA buffer, or None if out of bounds
pub fn buffer_offset(col: usize, row: usize) -> Option<usize> {
    if col >= BUFFER_WIDTH || row >= BUFFER_HEIGHT {
        return None;
    }
    Some((row * BUFFER_WIDTH + col) * 2)
}