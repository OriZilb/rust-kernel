
/// The enum represents the possible return values of an IRQ (Interrupt Request) handler.
/// It indicates whether the interrupt was handled by the handler or not.
#[derive(Copy, Clone, Debug)]
pub enum IrqResult {
    Handled,
    NotHandled,
}

/// Type alias for an IRQ handler function.
/// The function takes an IRQ number and a reference to the interrupt stack frame,
/// and returns an IrqResult indicating the outcome of the handling.
pub type IrqHandlerFn = fn(&InterruptStackFrame) -> IrqResult;

/// Number of handlers that can be registered per IRQ number.
pub const HANDLERS_PER_IRQ_NUMBER = 4;

/// Total number of IRQs supported.
pub const TOTAL_IRQS_NUMBER: usize = 16;