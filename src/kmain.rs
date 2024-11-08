use crate::vga::{Vga, VgaImpl};

const HELLO_WORLD: &'static str = concat!(
    "+------------------------------------------------------------------------------+",
    "| With Love! <3 - GetAGripGal :3                                               |",
    "|                                                                              |",
    "|                                                                              |",
    "|                                                                              |",
    "|                                                                              |",
    "|                                                                              |",
    "|                                                                              |",
    "|                                                                              |",
    "|                                                                              |",
    "|                                                                              |",
    "|                                                                              |",
    "|                                                                              |",
    "|                                                                              |",
    "|                                                                              |",
    "|                                                                              |",
    "|                                                                              |",
    "|                                                                              |",
    "|                                                                              |",
    "|                                                                              |",
    "|                                                                              |",
    "|                                                                              |",
    "|                                                                              |",
    "|                                                                              |",
    "+------------------------------------------------------------------------------+",
);

/// ------  ------
/// # Entrypoint #
/// ------  ------
#[no_mangle]
pub extern "C" fn kmain() {
    let vga = Vga::get_impl();
    vga.clear(' ', 0x17);
    vga.write_str(HELLO_WORLD, 0x17);
}