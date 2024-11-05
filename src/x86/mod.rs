use core::arch::asm;

/// Setup the headers for x86
#[macro_export]
macro_rules! arch_headers {
    () => {
        core::arch::global_asm!(
            "
        .global start
        
        .section .text
            .align 4
        "
        );

        multiboot_header!();
    };
}

/// Halt the cpu
pub unsafe fn cpu_halt() {
    asm!("hlt", options(nomem, nostack));
}

/// Setup stack space
pub unsafe fn setup_stack() {
    #[link_section = ".bss"]
    static mut STACK_SPACE: [u8; 8192] = [0; 8192];

    asm!("mov esp, {}", options(nomem, nostack), sym STACK_SPACE);
    asm!("cli", options(nomem, nostack));
}
