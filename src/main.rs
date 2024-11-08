//! main.rs needs to exist or else cargo will refuse to compile the binary
#![no_std]
#![no_main]

use core::panic::PanicInfo;
use galkernel::vga::{Vga, VgaImpl};

#[inline(never)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let vga = Vga::get_impl();
    vga.write_str(info.message().as_str().unwrap(), 0x07);
    loop {}
}
