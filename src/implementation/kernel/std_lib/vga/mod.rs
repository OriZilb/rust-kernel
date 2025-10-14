/**
 * VGA - video graphics array module.
 * Provides basic VGAChar output functionality to the screen using VGA VGAChar mode.
 */

mod vga;
mod consts;
mod unsafe_wrappers;
mod VGAChar;

pub use vga::*;