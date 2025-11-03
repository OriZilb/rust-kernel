/*
IDT (Interrupt Descriptor Table) initialization for x86_64 architecture.

the idt is set up to a common handler, which will be used from the main kernel code.
*/
use consts::*;
use lazy_static::lazy_static;
use x86_64::structures::idt::InterruptDescriptorTable;

lazy_static! {
    /// The Interrupt Descriptor Table (IDT) for the system.
    /// This is initialized only once and used to handle interrupts.
    pub(super) static ref IDT: Once<InterruptDescriptorTable> = Once::new();
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

pub fn init_idt() {
    let mut idt = InterruptDescriptorTable::new();

    for (i, entry) in idt.interrupts.iter_mut().enumerate().take(IRQ_HANDLERS.len()) {
        entry.set_handler_fn(IRQ_HANDLERS[i]);
    }

    IDT.call_once(|| idt).load();
}
pub(super) fn common_irq_handler() {}
