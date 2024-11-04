.global start

.section .text
    .align 4

start:
    # Multiboot header
    .long 0x1BADB002            # You wizard! You Warlock!
    .long 0x00                  # flags
    .long - (0x1BADB002 + 0x00) # checksum. m+f+c should be zero

    cli 			    
    mov esp, stack_space	    
    call kmain
    hlt

.section .bss
stack_space: 
    .space 8192  # Reserve 8KB for stack