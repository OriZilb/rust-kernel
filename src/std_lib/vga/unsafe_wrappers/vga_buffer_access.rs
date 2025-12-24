/**
 * Unsafe wrappers around VGA functions
 * These functions perform low-level operations on the VGA vgachar buffer
 * The idea is to encapsulate unsafe code in one place, with all the necessary safety checks,
 * and to provide abstracted safe interfaces elsewhere.
 */
use super::super::consts::*;
use super::super::vgachar::*;
use super::utils::buffer_offset;
use core::ptr;

/// Read a character from the VGA buffer at the specified column and row
///
/// # Safety
/// This function performs raw pointer arithmetic and reads directly from the VGA buffer.
/// Buffer overflow is checked at the buffer_offset function.
///
/// # Arguments
/// * `col` - Column index (0 to BUFFER_WIDTH - 1)
/// * `row` - Row index (0 to BUFFER_HEIGHT - 1)
/// # Returns
/// * vgachar` - The character byte at the specified position with its color coding, or None if out of bounds
pub fn read_char(position: Position) -> Option<VGAChar> {
    let offset = buffer_offset(position);
    match offset {
        None => None,
        Some(o) => unsafe { Some(*VGA_BUFFER_ADDRESS.add(o)) },
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
pub fn write_char(position: Position, char: VGAChar) -> Result<(), &'static str> {
    let offset = buffer_offset(position);
    match offset {
        None => Err("Position out of bounds"),
        Some(o) => unsafe {
            ptr::write_volatile(VGA_BUFFER_ADDRESS.add(o), char);
            Ok(())
        },
    }
}

/// Copy a character from one position to another in the VGA buffer
/// # Safety
/// This function performs raw pointer arithmetic and reads/writes directly to the VGA buffer.
/// Buffer overflow is checked at the buffer_offset function.
///
/// # Arguments
/// * src_position - Source position (column and row)
/// * dest_position - Destination position (column and row)
/// # Returns
/// * `Result<(), &'static str>` - Ok(()) if successful, Err message if out of bounds
pub fn copy_char(src_position: Position, dest_position: Position) -> Result<(), &'static str> {
    let src_offset = buffer_offset(src_position);
    let dest_offset = buffer_offset(dest_position);
    match (src_offset, dest_offset) {
        (Some(src_o), Some(dest_o)) => unsafe {
            ptr::write_volatile(
                VGA_BUFFER_ADDRESS.add(dest_o),
                *VGA_BUFFER_ADDRESS.add(src_o),
            );
            Ok(())
        },
        _ => Err("Source or destination position out of bounds"),
    }
}
