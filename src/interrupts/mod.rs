use crate::arch::DISPATCH_TABLE;
use crate::interrupts::definitions::{IrqHandlerFn, IrqHandlerName};

pub mod definitions;
pub mod dispatch_table;

/* the files here (definitions, dispatch_table) are somewhat of a mess. The files needs to be
organized better, and I need to define what is in each file.
 */

pub fn register_driver(irq_num: usize, handler: IrqHandlerFn, name: IrqHandlerName) -> bool {
    DISPATCH_TABLE.lock().register_handler(irq_num, handler, name)
}

pub fn unregister_driver(irq_num: usize, name: IrqHandlerName) -> bool {
    DISPATCH_TABLE.lock().unregister_handler(irq_num, name)
}