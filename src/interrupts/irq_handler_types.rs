/*
Interrupt handlers structs for convenient management.

Interrupt handlers function should have a specific signature, with specific output type representing 
whether the interrupt was handled or not.

A handler should also have a name for identification purposes. The name is defined as a heapless 
String with a maximum size.
*/

use crate::arch::CpuState; // arch-specific CPU state representation given at interrupt time
use heapless::String;
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum IrqResult {
    Handled,
    NotHandled,
}
pub type IrqHandlerFn = fn(&CpuState) -> IrqResult;

pub const MAX_IRQ_HANDLER_NAME_SIZE: usize = 20;
pub type IrqHandlerName = String<MAX_IRQ_HANDLER_NAME_SIZE>;

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



