/**
 * Unsafe wrappers around VGA functions
 * These functions perform low-level operations on the VGA text buffer
 * The idea is to encapsulate unsafe code in one place, with all the necessary safety checks,
 * and to provide abstracted safe interfaces elsewhere.
 */
use super::super::consts::*;
use super::utils::buffer_offset;
use super::super::text::*;

/// Read a character from the VGA buffer at the specified column and row
///
/// # Safety
/// This function performs raw pointer arithmetic and reads directly from the VGA buffer.
///
/// # Arguments
/// * `col` - Column index (0 to BUFFER_WIDTH - 1)
/// * `row` - Row index (0 to BUFFER_HEIGHT - 1)
/// # Returns
/// * VGAChar` - The character byte at the specified position with its color coding, or None if out of bounds
pub fn read_char_at(col: usize, row: usize) -> Option<u8> {
    let offset = buffer_offset(col, row);
    match offset{
        None => None,
        Some(o) => unsafe {
            Some(*VGA_BUFFER_ADDRESS.add(o))
        },
    }
}

/// Write a character to the VGA buffer at the specified column and row with the given color
/// # Safety
/// This function performs raw pointer arithmetic and writes directly to the VGA buffer.
///
/// # Arguments
/// * `col` - Column index (0 to BUFFER_WIDTH - 1)
/// * `row` - Row index (0 to BUFFER_HEIGHT - 1)
/// * `byte` - The ASCII character byte to write
/// * `color_code` - The color code byte (foreground and background combined)
/// # Returns
/// * `Result<(), &'static str>` - Ok(()) if successful, Err message if out of bounds
pub fn write_char_at(col: usize, row: usize, byte: u8, color_code: u8) -> Result<(), &'static str> {
    if col >= BUFFER_WIDTH || row >= BUFFER_HEIGHT {
        return Err("Column or row out of bounds");
    }
    let offset = buffer_offset(col, row).unwrap();
    
}