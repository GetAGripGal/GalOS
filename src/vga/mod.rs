const VGA_PHYSICAL_ADDRESS: u64 = 0xb8000;

const LINES: usize = 25;
const COLLUMS: usize = 80;
const ELEMENT_SIZE: usize = 2;
const VIDEO_MEMORY_ELEMENT_COUNT: usize = LINES * COLLUMS;
const VIDEO_MEMORY_SIZE: usize = VIDEO_MEMORY_ELEMENT_COUNT * ELEMENT_SIZE;

/// Handles the vga display
pub struct Vga;

impl Vga {
    /// Write a string to video memory
    pub fn write_str(what: &str) {
        // A pointer to the video memory buffer
        let video_memory_ptr = VGA_PHYSICAL_ADDRESS as *mut u8;

        // Create a slice of the video memory
        // # SAFETY
        // Since the slice will prevent us from writing outside of the given bounds, we will never write outside of video memory
        let video_memory: &mut [u8] =
            unsafe { core::slice::from_raw_parts_mut(video_memory_ptr, VIDEO_MEMORY_SIZE) };

        // Clear the screen
        let mut i = 0;
        while i < VIDEO_MEMORY_SIZE {
            video_memory[i] = b' ';
            video_memory[i + 1] = 0x07; // black background, ligh-gray foreground
            i += ELEMENT_SIZE;
        }

        i = 0;
        let chars = what.as_bytes();
        for j in 0..chars.len() {
            video_memory[i] = chars[j];
            video_memory[i + 1] = 0x07; // black background, ligh-gray foreground
            i += ELEMENT_SIZE;
            if i >= VIDEO_MEMORY_SIZE {
                break;
            }
        }
    }
}
