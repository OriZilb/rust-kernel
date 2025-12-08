/*
IDT (Interrupt Descriptor Table) initialization for x86_64 architecture.

The idt is set up to a common handler, which will be used from the main kernel code.
The idea is to have a hardware independent common handler that will delegate the handling
to appropriate sub-handlers that can be dynamically changed by the kernel.

The common_irq_handler will get the irq number, and will have a dynamic dispatch table
to call the appropriate handler for that irq. Each irq number could contain up to four handlers,
as defined in the kernel/interrupts/definitions.rs file.
*/
use consts::*;
use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use dispatch_table::IrqDispatchTable;
use spin::Once;

lazy_static! {
    /// The Interrupt Descriptor Table (IDT) for the system.
    /// This is initialized only once and used to handle interrupts.
    pub(super) static ref IDT: Once<InterruptDescriptorTable> = Once::new();

    /// The IRQ Dispatch Table that holds the registered IRQ handlers.
    /// This table allows dynamic registration and unregistration of IRQ handlers.
    pub(super) static ref DISPATCH_TABLE: IrqDispatchTable = IrqDispatchTable::new();
}

/// Generate automatically IRQ handlers for IRQs 0 to 15
/// Each handler calls the common_irq_handler with its IRQ number
/// and the interrupt stack frame.
macro_rules! generate_irq_handlers {
    ($($num:expr),*) => {
        $(
            extern "x86-interrupt" fn irq$num(stack_frame: InterruptStackFrame) {
                common_irq_handler(&mut stack_frame, $num);
            }
        )*

        // array of function pointers
        const IRQ_HANDLERS: &[extern "x86-interrupt" fn(InterruptStackFrame)] = &[
            $(irq$num),*
        ];
    };
}

generate_irq_handlers!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);

/// Initialize the Interrupt Descriptor Table (IDT) with IRQ handlers.
/// Essentially, this function uses the macro generated wrappers for the common_irq_handler to
/// pass the IRQ number along with the interrupt stack frame to the handler.
pub fn init_idt() {
    let mut idt = InterruptDescriptorTable::new();

    for (i, entry) in idt
        .interrupts
        .iter_mut()
        .enumerate()
        .take(IRQ_HANDLERS.len())
    {
        entry.set_handler_fn(IRQ_HANDLERS[i]);
    }

    IDT.call_once(|| idt).load();
}

/// Common IRQ handler that processes interrupts based on their IRQ number.
/// This function delegates the responsibility of handling specific IRQs to appropriate sub-handlers,
/// that can be dynamically changed by the kernel, in a hardware independent way.
/// # Arguments
/// * `stack_frame` - The interrupt stack frame containing CPU state at the time of the interrupt.
/// * `irq_num` - The IRQ number of the interrupt being handled.
pub(super) fn common_irq_handler(stack_frame: InterruptStackFrame, irq_num: int) {
    DispatchTable.entries[irq_num].run_handler(&stack_frame);
}
