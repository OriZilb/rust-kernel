use core::panic::PanicInfo;
use crate::std_lib::vga::*;
#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    print("\n\n");
    println("Kernel Panic!");
    println("System halted.");
    loop {}
}
