#![no_std]
#![no_main]

// Inline boot assembly
global_asm!(include_str!("boot.asm"));

pub mod vga;

use core::{arch::global_asm, panic::PanicInfo};
use vga::Vga;

#[no_mangle]
pub extern "C" fn kmain() -> ! {
    Vga::write_str("With Love! <3 - GetAGripGal");
    loop {}
}

#[inline(never)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    Vga::write_str(info.message().as_str().unwrap());
    loop {}
}
