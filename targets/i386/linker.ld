OUTPUT_FORMAT(elf32-i386)
ENTRY(start)
SECTIONS
{
    . = 0x100000;
    .multiboot : { *(.multiboot) }
    .text : { *(.text) }
    .data : { *(.data) }
    .bss  : { 
        *(.bss)
        *(COMMON) 
    }
    .bss.stack (NOLOAD) : {
        . = ALIGN(16);
        stack_space = .;
        resb 32768;
    }
    .rodata : { *(.rodata) }
}