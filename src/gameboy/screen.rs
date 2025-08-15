use crate::gameboy::ram::Memory;

const PIXELS: usize = 160 * 144;

#[derive(Default, Debug)]
pub struct Screen {
    // TODO: Consider using a different abstraction.
    // pixels: [u8; 160*144]
    pixels: Vec<u8>,
}

impl Screen {
    pub fn new() -> Self {
        Self {
            pixels: vec![0; PIXELS],
        }
    }

    pub fn update(&mut self, _ram: &Memory) {
        // For now, we just fill the screen with a solid color for testing.
        for pixel in &mut self.pixels {
            *pixel = 0xFF; // White color
        }
    }
}
