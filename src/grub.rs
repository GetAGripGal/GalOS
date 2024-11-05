/// Setup the grub multiboot header
#[macro_export]
macro_rules! multiboot_header {
    () => {
        core::arch::global_asm!(
            ".section multiboot
        # Multiboot header
        .long 0x1BADB002            # You wizard! You Warlock!
        .long 0x00                  # flags
        .long - (0x1BADB002 + 0x00) # checksum. m+f+c should be zero"
        );
    };
}
