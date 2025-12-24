pub mod interrupts;
pub mod drivers;
pub type CpuState = x86_64::structures::idt::InterruptStackFrame;