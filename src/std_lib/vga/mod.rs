/**
 * VGA - video graphics array module.
 * Provides basic vgachar output functionality to the screen using VGA vgachar mode.
 */
mod consts;
mod unsafe_wrappers;
mod vgachar;
mod writer;

pub use vgachar::*;
pub use writer::*;
