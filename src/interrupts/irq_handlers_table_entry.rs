/*
The irq handling architecture uses a dispatch table to manage multiple handlers for each IRQ number.
Each entry in the dispatch table corresponds to a specific IRQ number and contains multiple slots
for handlers.

The functionality provided by this module includes registering, unregistering, and running handlers.
Note that by design, all the handlers registered for a specific IRQ number are called when that IRQ occurs.
Normally, one would expect only one handler to handle the interrupt, but this design allows for more flexibility.
Maybe in the future we will log it for debugging purposes.

That means that each handler is required to check whether it should handle the interrupt or not,
and return the appropriate result.
*/
use crate::arch::CpuState;
use crate::interrupts::irq_handler_types::{IrqHandler, IrqHandlerFn, IrqHandlerName, IrqResult};
pub struct IrqHandlersEntry<const HANDLERS_PER_IRQ_NUMBER: usize> {
    pub handlers: [Option<IrqHandler>; HANDLERS_PER_IRQ_NUMBER]
}

impl<const HANDLERS_PER_IRQ_NUMBER: usize> IrqHandlersEntry<HANDLERS_PER_IRQ_NUMBER> {
    pub const fn new() -> Self {
        Self {
            handlers: [const{None}; HANDLERS_PER_IRQ_NUMBER],
        }
    }

    /// Registers a new IRQ handler in the entry.
    /// Returns true if the handler was successfully registered, false if there was no space.
    pub fn register_handler(&mut self, handler: IrqHandlerFn, name: IrqHandlerName) -> bool {
        for slot in self.handlers.iter_mut() {
            if slot.is_none() {
                *slot = Some(IrqHandler::new(name, handler));
                return true;
            }
        }
        false
    }

    /// Unregisters an IRQ handler from the entry.
    /// Returns true if the handler was found and unregistered, false otherwise.
    pub fn unregister_handler(&mut self, handler_name: IrqHandlerName) -> bool {
        for slot in self.handlers.iter_mut() {
            if let Some(registered_handler) = slot {
                if *registered_handler.get_name() == handler_name {
                    *slot = None;
                    return true;
                }
            }
        }
        false
    }

    /// Run the handler for this entry. Returns IrqResult::Handled if any handler handled the interrupt,
    /// otherwise IrqResult::NotHandled. This functions calls each registered handler in order until
    /// one handles the interrupt. When a handler returns IrqResult::Handled, the function stops and
    /// returns that result.
    pub fn run_handler(&self, stack_frame: &CpuState) -> IrqResult {
        let mut handled: IrqResult = IrqResult::NotHandled;
        for slot in self.handlers.iter() {
            if let Some(registered_handler) = slot {
                let result = registered_handler.get_handler()(stack_frame);
                if result == IrqResult::Handled {
                    handled = IrqResult::Handled;
                }
            }
        }
        handled
    }
}
