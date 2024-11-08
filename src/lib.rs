#![no_std]
#![no_main]

pub mod arch;
pub mod vga;

use crate::vga::{Vga, VgaImpl};
use core::panic::PanicInfo;

const HELLO_WORLD: &'static str = concat!("With Love! <3 - GetAGripGal");

/// ------  ------
/// # Entrypoint #
/// ------  ------
#[no_mangle]
pub extern "C" fn kmain() {
    let vga = Vga::get_impl();
    vga.clear(' ', 0x17);
    vga.write_str(HELLO_WORLD, 0x17);
}

#[inline(never)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let vga = Vga::get_impl();
    vga.write_str(info.message().as_str().unwrap(), 0x07);
    loop {}
}
