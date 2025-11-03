use pc_keyboard::layouts::Us104Key;
use pc_keyboard::{Keyboard, ScancodeSet1};
use pic8259::ChainedPics;
use spin::{Mutex, Once};
use x86_64::structures::idt::InterruptDescriptorTable;

/// The keyboard handler instance protected by a Mutex for synchronized access.
/// This ensures that only one thread can access the keyboard state at a time.
pub(super) static KEYBOARD: Mutex<Option<Keyboard<Us104Key, ScancodeSet1>>> = Mutex::new(None);

/// The I/O port addresses for the master and slave Programmable Interrupt Controllers (PICs).
pub(super) const MASTER_CONTROL: u8 = 0x20;
pub(super) const MASTER_DATA: u8 = 0x21;

pub(super) const SLAVE_CONTROL: u8 = 0xA0;
pub(super) const SLAVE_DATA: u8 = 0xA1;

/// The base vector numbers in the IDT for the master and slave PICs.
pub(super) const PIC_1_OFFSET: u8 = 32;
pub(super) const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;
pub static PICS: Mutex<ChainedPics> =
    Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

/// Enum representing the IRQ numbers for various hardware interrupts.
#[derive(Clone, Copy)]
pub(super) enum IrqNumber {
    SystemTimer,
    Keyboard,
    InterruptController,
    SerialPort2,
    SerialPort1,
    ParallelPort2,
    FloppyDisk,
    ParallelPort1,
    RealTimeClock,
    ACPI,
    Reserved1,
    Reserved2,
    Mouse,
    FPU,
    PrimaryATA,
    SecondaryATA,
}
