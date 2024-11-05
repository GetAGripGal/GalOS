#![no_std]
#![no_main]

pub mod grub;
pub mod kmain;
pub mod vga;

#[cfg(target_arch = "x86")]
#[macro_use]
pub mod x86;

use kmain::kmain;
use x86::{cpu_halt, setup_stack};

arch_headers!(); // Architecture specific headers

#[no_mangle]
pub unsafe extern "C" fn start() -> ! {
    setup_stack();
    kmain();
    cpu_halt();
    loop {}
}
