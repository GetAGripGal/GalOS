use core::arch::{asm, global_asm};

global_asm!(
    "
.global start

.section .text
    .align 4

.section multiboot
    # Multiboot header
    .long 0x1BADB002            # You wizard! You Warlock!
    .long 0x00                  # flags
    .long - (0x1BADB002 + 0x00) # checksum. m+f+c should be zero
    "
);

#[link_section = ".bss"]
static mut STACK_SPACE: [u8; 32768] = [0; 32768];

#[no_mangle]
pub unsafe extern "C" fn start() -> ! {
    setup_stack();
    crate::kmain();
    cpu_halt();
    loop {}
}

/// Halt the cpu
unsafe fn cpu_halt() {
    asm!("hlt", options(nomem, nostack));
}

/// Setup stack space
unsafe fn setup_stack() {
    asm!("mov esp, {}", options(nomem, nostack), sym STACK_SPACE);
    asm!("cli", options(nomem, nostack));
}
