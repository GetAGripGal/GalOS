use core::marker::PhantomData;
use spin::{mutex::Mutex, MutexGuard};

use crate::vga::VgaImpl;

const VGA_PHYSICAL_ADDRESS: u64 = 0xb8000;

const LINES: usize = 25;
const COLLUMS: usize = 80;
const ELEMENT_SIZE: usize = 2;
const VIDEO_MEMORY_ELEMENT_COUNT: usize = LINES * COLLUMS;
const VIDEO_MEMORY_ITEM_COUNT: usize = VIDEO_MEMORY_ELEMENT_COUNT * ELEMENT_SIZE;

/// The i386 implementation of the vga handler
pub struct Vgai386(PhantomData<()>);

impl Vgai386 {
    /// Get the instance of Vgai386
    /// # WARNING
    /// Blocks until the mutex is free,
    pub fn get<'a>() -> MutexGuard<'a, Self> {
        static ONCE: Mutex<Vgai386> = Mutex::new(Vgai386(PhantomData));
        ONCE.lock()
    }
}

impl VgaImpl for Vgai386 {
    /// Return a slice to the video memory
    fn video_memory<'a>(&'a self) -> &'a mut [u8] {
        // Create a slice of the video memory
        // # SAFETY
        // Since the slice will prevent us from writing outside of the given bounds, we will never write outside of video memory.
        // It is also the case that only one reference and instance of Vgai386 can exist at the same time, meaning the memory will never be addressed twice
        unsafe {
            core::slice::from_raw_parts_mut(
                VGA_PHYSICAL_ADDRESS as *mut u8,
                VIDEO_MEMORY_ITEM_COUNT,
            )
        }
    }

    /// Clear vga memory buffer with a character
    fn clear(&self, c: char, attr: u8) {
        let video_memory = self.video_memory();

        // Clear the screen
        let mut i = 0;
        while i < video_memory.len() {
            video_memory[i] = c as u8;
            video_memory[i + 1] = attr;
            i += ELEMENT_SIZE;
        }
    }

    /// Write a string to video memory
    fn write_str(&self, what: &str, attr: u8) {
        let video_memory = self.video_memory();

        let mut i = 0;
        let chars = what.as_bytes();
        for c in chars {
            video_memory[i] = *c;
            video_memory[i + 1] = attr;
            i += ELEMENT_SIZE;
            if i >= video_memory.len() {
                break;
            }
        }
    }
}
