use super::super::consts::*;

/// Calculate the byte offset in the VGA vgachar buffer for a given column and row.
/// Each character cell in VGA vgachar mode consists of 2 bytes: one for the ASCII character
/// and one for the color attribute (foreground and background colors).
/// # Arguments
/// * `col` - Column index (0 to BUFFER_WIDTH - 1)
/// * `row` - Row index (0 to BUFFER_HEIGHT - 1)
/// # Returns
/// * `Option<usize>` - The byte offset in the VGA buffer, or None if out of bounds
pub fn buffer_offset(position: Position) -> Option<usize> {
    // note that position.col and position.row are usize, so they cannot be negative
    if position.col >= BUFFER_WIDTH || position.row >= BUFFER_HEIGHT {
        return None;
    }
    Some(position.row * BUFFER_WIDTH + position.col)
}
