use lazy_static::lazy_static;
use pic8259::ChainedPics;
use spin::Mutex;
use crate::arch::x86_64::interrupts::idt::init_idt;
pub(super) const PIC_1_OFFSET: u8 = 32;
pub(super) const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

lazy_static! {
    /// The chained Programmable Interrupt Controllers (PICs) for the x86_64 architecture.
    pub(super) static ref PICS: Mutex<ChainedPics> =
        Mutex::new(unsafe {ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET)});
}

/// Initialize the Programmable Interrupt Controllers (PICs).
/// This function sets up the PICs and masks all IRQs initially.
pub(super) unsafe fn init_pic() {
    let mut pics = PICS.lock();
    pics.initialize();
    pics.write_masks(0xff, 0xff); // Mask all IRQs initially
}

/// Initialize the interrupt system by setting up the PICs and IDT.
/// This function unmasks all IRQs after initialization.
pub unsafe fn initialize_interrupts() {
    init_pic();
    init_idt();
    x86_64::instructions::interrupts::enable();
    PICS.lock().write_masks(0, 0); // Unmask all IRQs
}