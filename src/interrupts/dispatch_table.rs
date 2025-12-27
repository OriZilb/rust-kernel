/*
This file defines the IRQ dispatch table and related structures for managing IRQ handlers.
It allows registering, unregistering, and running IRQ handlers for specific IRQ numbers.

This file is for use by the architecture-specific interrupt handling code.
The architecture-specific code is responsible for initializing the dispatch table
and forwarding the interrupts to the appropriate handlers in this table.
 */
use crate::arch::CpuState;
use crate::interrupts::definitions::*;

/// Irq handler is a struct which contains the handler function the interrupt handler name.
/// The name is used to identify the handler when unregistering it.
pub struct IrqHandler{
    handler: IrqHandlerFn,
    name: IrqHandlerName,
}

impl IrqHandler {
    pub fn new(name: IrqHandlerName, handler: IrqHandlerFn) -> Self {
        Self {
            handler,
            name,
        }
    }

    pub fn get_name(&self) -> &IrqHandlerName {
        &self.name
    }

    pub fn get_handler(&self) -> IrqHandlerFn {
        self.handler
    }
}

/// Represents a collection of IRQ handlers for a specific IRQ number.
pub struct IrqHandlersEntry{
    pub handlers: [Option<IrqHandler>; HANDLERS_PER_IRQ_NUMBER]
}

impl IrqHandlersEntry {
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
        for slot in self.handlers.iter() {
            if let Some(registered_handler) = slot {
                let result = registered_handler.get_handler()(stack_frame);
                if result == IrqResult::Handled {
                    return IrqResult::Handled;
                }
            }
        }
        IrqResult::NotHandled
    }
}

/// Represents the IRQ dispatch table containing handlers for all IRQ numbers.
pub struct IrqDispatchTable {
    pub entries: [IrqHandlersEntry; TOTAL_IRQS_NUMBER],
    pub entry_count: usize,
}

impl IrqDispatchTable {
    pub const fn new() -> Self {
        Self {
            entries: [const{IrqHandlersEntry::new()}; TOTAL_IRQS_NUMBER],
        }
    }

    pub fn register_handler(&mut self, irq_num: usize, handler: IrqHandlerFn, name: IrqHandlerName) -> bool {
        if irq_num < TOTAL_IRQS_NUMBER {
            self.entries[irq_num].register_handler(handler, name)
        } else {
            false
        }
    }

    pub fn unregister_handler(&mut self, irq_num: usize, name: IrqHandlerName) -> bool {
        if irq_num < TOTAL_IRQS_NUMBER {
            self.entries[irq_num].unregister_handler(name)
        } else {
            false
        }
    }
}