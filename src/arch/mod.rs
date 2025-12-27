pub mod x86_64;

#[cfg(target_arch = "x86_64")]
pub use x86_64::*;

// For future architectures support we need to insert the needed cfg attributes and re-exports here.
// Generally, each architecture should have the same submodules exports as x86_64 above to have a consistent API.
// This way the kernel would compile cleanly for different architectures.