/*
The IRQ dispatch table. The hardware specific code will use this table to forward IRQs to the
appropriate handlers. The dispatch table allows us flexibility of dynamically registering and
unregistering handlers for specific IRQ numbers.
 */
use crate::arch::CpuState;
use crate::interrupts::irq_handlers_table_entry::IrqHandlersEntry;
use crate::interrupts::irq_handler_types::{IrqHandlerFn, IrqHandlerName};
pub struct IrqDispatchTable<const HANDLERS_PER_IRQ_NUMBER: usize, const TOTAL_IRQS_NUMBER: usize> {
    entries: [IrqHandlersEntry<HANDLERS_PER_IRQ_NUMBER>; TOTAL_IRQS_NUMBER],
}

impl<const HANDLERS_PER_IRQ_NUMBER: usize, const TOTAL_IRQS_NUMBER: usize>
    IrqDispatchTable<HANDLERS_PER_IRQ_NUMBER, TOTAL_IRQS_NUMBER>
{
    pub const fn new() -> Self {
        Self {
            entries: [const { IrqHandlersEntry::new() }; TOTAL_IRQS_NUMBER],
        }
    }

    /// Registers a new IRQ handler for the given IRQ number.
    /// Returns true if the handler was successfully registered, false otherwise.
    pub fn register_handler(
        &mut self,
        irq_num: usize,
        handler: IrqHandlerFn,
        name: IrqHandlerName,
    ) -> bool {
        if irq_num < TOTAL_IRQS_NUMBER {
            self.entries[irq_num].register_handler(handler, name)
        } else {
            false
        }
    }

    /// Unregisters a new IRQ handler for the given IRQ number.
    /// Returns true if the handler was successfully unregistered, false otherwise.
    pub fn unregister_handler(&mut self, irq_num: usize, name: IrqHandlerName) -> bool {
        if irq_num < TOTAL_IRQS_NUMBER {
            self.entries[irq_num].unregister_handler(name)
        } else {
            false
        }
    }

    /// Run the handlers for a given IRQ number.
    pub fn run_handler(&self, irq_num: usize, stack_frame: &CpuState) {
        if irq_num < TOTAL_IRQS_NUMBER {
            self.entries[irq_num].run_handler(stack_frame);
        }
    }
}
