/*
Linker script for x86_64 architecture.
This script defines the memory layout for the kernel, starting at the 1MB mark.
*/

/*
The ENTRY directive specifies the entry point of the program,
which is the start symbol defined in rust-kernel/src/kernel/arch/x86_64/main.asm
*/
ENTRY(start)

SECTIONS
{
    /*
    This command sets the current address to be 1MB,
    practically setting the start address of the kernel to be 1MB.
    This is the standard load address for multiboot compatible kernels.
    */
    . = 1M;

    /*
    The first section is the .boot section which contains the multiboot header. It takes all the input files with
    sections named .multiboot_header and places them in the output file's .boot section. The KEEP command ensures
    that this section is optimized out by the linker, even if it appears to be unused. This section will be used by
    the GRUB bootloader to load the kernel.
    */
    .boot :
    {
        KEEP(*(.multiboot_header))
    }

    /*
    The next section is the .text section, which is defined at
    rust-kernel/src/kernel/arch/x86_64/main.asm and main64.asm in the
    same directory. This section contains the actual code of the kernel. As mentioned earlier the entry point will be
    in the start symbol which is defined in main.asm, but at the end of the .text section of main.asm we jump to the
    .text section of main64.asm to continue execution in 64-bit mode.
    */
    .text :
    {
        *(.text)
    }
}