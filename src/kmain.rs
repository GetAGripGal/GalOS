use core::panic::PanicInfo;

use crate::vga::Vga;

/// ------  ------
/// # Entrypoint #
/// ------  ------
#[no_mangle]
pub extern "C" fn kmain() {
    Vga::write_str("With Love! <3 - GetAGripGal");
}

#[inline(never)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    Vga::write_str(info.message().as_str().unwrap());
    loop {}
}
