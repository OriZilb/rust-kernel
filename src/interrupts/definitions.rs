use crate::arch::CpuState;
use heapless::String;

/// The enum represents the possible return values of an IRQ (Interrupt Request) handler.
/// It indicates whether the interrupt was handled by the handler or not.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum IrqResult {
    Handled,
    NotHandled,
}

/// Type alias for an IRQ handler function.
/// The function takes an IRQ number and a reference to the interrupt stack frame,
/// and returns an IrqResult indicating the outcome of the handling.
pub type IrqHandlerFn = fn(&CpuState) -> IrqResult;

/// Number of handlers that can be registered per IRQ number.
pub const HANDLERS_PER_IRQ_NUMBER: usize = 4;

/// Total number of IRQs supported.
pub const TOTAL_IRQS_NUMBER: usize = 16;

/// Irq handler names are heapless String of up to the following size
pub(super) const MAX_IRQ_HANDLER_NAME_SIZE: usize = 20;
pub(super) type IrqHandlerName = String<MAX_IRQ_HANDLER_NAME_SIZE>;

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
