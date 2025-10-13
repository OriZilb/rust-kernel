/**
 * VGA - video graphics array module.
 * Provides basic text output functionality to the screen using VGA text mode.
 */

mod vga;
mod consts;
mod unsafe_wrappers;
mod text;

pub use vga::*;