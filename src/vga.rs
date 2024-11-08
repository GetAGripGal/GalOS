use spin::MutexGuard;

/// Handles the vga display
pub struct Vga;

/// A trait for implementations of the vga handle
pub trait VgaImpl {
    /// Return a slice to the video memory
    fn video_memory<'a>(&'a self) -> &'a mut [u8];
    /// Clear vga memory buffer with a character
    fn clear(&self, c: char, attr: u8);
    /// Write a string to video memory
    fn write_str(&self, what: &str, attr: u8);
}

impl Vga {
    /// Get the vga implementation
    /// # Warning
    /// Blocks until the vga handle is free
    pub fn get_impl<'a>() -> MutexGuard<'a, impl VgaImpl> {
        if cfg!(target_arch = "x86") {
            return crate::arch::i386::vga::Vgai386::get();
        }
        panic!("VGA text mode not supported on target arch")
    }
}
