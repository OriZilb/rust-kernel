use crate::kernel::interrupts::definitions::{HANDLERS_PER_IRQ_NUMBER, IrqHandlerFn, TOTAL_IRQS_NUMBER};

pub(super) struct IrqHandler{
    handler: IrqHandlerFn,
    name: str,
}

impl IrqHandler {
    pub(super) fn new(name: &str, handler: IrqHandlerFn) -> Self {
        Self {
            handler,
            name,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_handler(&self) -> IrqHandlerFn {
        self.handler
    }
}

/// Represents a collection of IRQ handlers for a specific IRQ number.
pub(super) struct IrqHandlersEntry{
    pub handlers: [Option<IrqHandler>; HANDLERS_PER_IRQ_NUMBER]
}

impl IrqHandlersEntry {
    pub const fn new() -> Self {
        Self {
            handlers: [None; HANDLERS_PER_IRQ_NUMBER],
        }
    }

    /// Registers a new IRQ handler in the entry.
    /// Returns true if the handler was successfully registered, false if there was no space.
    pub fn register_handler(&mut self, handler: IrqHandlerFn, name: &str) -> bool {
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
    pub fn unregister_handler(&mut self, handler: IrqHandlerFn) -> bool {
        for slot in self.handlers.iter_mut() {
            if let Some(registered_handler) = slot {
                if *registered_handler.get_name() == handler.get_name() {
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
    pub fn run_handler(&self, stack_frame: &InterruptStackFrame) -> IrqResult {
        for slot in self.handlers.iter() {
            if let Some(registered_handler) = slot {
                let result = (registered_handler.get_handler())(stack_frame);
                if result == IrqResult::Handled {
                    return IrqResult::Handled;
                }
            }
        }
        IrqResult::NotHandled
    }
}

/// Represents the IRQ dispatch table containing handlers for all IRQ numbers.
pub(super) struct IrqDispatchTable {
    pub entries: [IrqHandlersEntry; TOTAL_IRQS_NUMBER]
}

impl IrqDispatchTable {
    pub const fn new() -> Self {
        Self {
            entries: [IrqHandlersEntry::new(); TOTAL_IRQS_NUMBER],
        }
    }
}