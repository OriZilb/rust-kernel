use cfg_if::cfg_if;
pub mod x86_64;

cfg_if! {
    if #[cfg(target_arch = "x86_64")] {
        pub type CpuState = x86_64::CpuState;
        pub use x86_64::interrupts::initializations::initialize_interrupts;
        pub use x86_64::interrupts::idt::DISPATCH_TABLE;
        pub use x86_64::drivers::keyboard;
    }
}