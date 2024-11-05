OUTPUT_FORMAT(elf32-i386)
ENTRY(start)
SECTIONS
{
    . = 0x100000;
    .multiboot : { KEEP(*(.multiboot)) }
    .text : { *(.text) }
    .data : { *(.data) }
    .bss  : { *(.bss), *(COMMON) }
    .bss.stack (NOLOAD) : {
        . = ALIGN(16);
        stack_space = .;
        resb 8192;
    }
    .rodata : { *(.rodata) }
}