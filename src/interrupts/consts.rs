/// Number of handlers that can be registered per IRQ number.
pub const HANDLERS_PER_IRQ_NUMBER: usize = 4;

/// Enum representing the IRQ numbers for various hardware interrupts.
#[derive(Clone, Copy)]
pub enum IrqNumber {
    SystemTimer,
    Keyboard,
    InterruptController,
    SerialPort2,
    SerialPort1,
    ParallelPort2,
    FloppyDisk,
    ParallelPort1,
    RealTimeClock,
    ACPI,
    Reserved1,
    Reserved2,
    Mouse,
    FPU,
    PrimaryATA,
    SecondaryATA,
}
