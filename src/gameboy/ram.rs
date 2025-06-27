/*
 * General memory map:
 *
 * Interrupt Enable Register
 * --------------------------- 0xFFFF
 * Internal RAM
 * --------------------------- 0xFF80
 * Empty but unusable for I/O
 * --------------------------- 0xFF4C
 * I/O ports
 * --------------------------- 0xFF00
 * Empty but unusable for I/O
 * --------------------------- 0xFEA0
 * Sprite Attrib Memory (OAM)
 * --------------------------- 0xFE00
 * Echo of 8kB Internal RAM
 * --------------------------- 0xE000
 * 8kB Internal RAM
 * --------------------------- 0xC000
 * 8kB switchable RAM bank
 * --------------------------- 0xA000
 * 8kB Video RAM
 * --------------------------- 0x8000 --
 * 16kB switchable ROM bank             |
 * --------------------------- 0x4000   |= 32kB Cartridge
 * 16kB ROM bank #0                     |
 * --------------------------- 0x0000 --
 * * b = bit, B = byte
 *
 * Writing at E000-FE00 writes at C000-DE00
 * Writing at C000-DE00 writes at C000-DE00
 *
 */

use std::fmt;

use crate::gameboy::rom::Rom;

const MEMORY_SIZE: usize = 64 * 1024;
const START_ROM_BANK: usize = 0x0000;

/// Represents the mirroring secton of the memory.
const MIRRORING_RANGE: std::ops::RangeInclusive<usize> = 0xE000..=0xFE00;
const MIRRORING_OFFSET: usize = 0xE000 - 0xC000;

pub(crate) struct Memory {
    pub ram: [u8; MEMORY_SIZE],
}

impl Memory {
    /// Creates a new instance of Memory with all bytes initialized to zero.
    pub fn new() -> Memory {
        Self {
            ram: [0; MEMORY_SIZE],
        }
    }

    /// Creates memory and load given rom.
    pub fn new_with_rom(rom: Rom) -> Memory {
        let mut memory = Self::new();
        let bytes_written = memory
            .write_slice(START_ROM_BANK, &rom.bytecode)
            .expect("Failed to write ROM data to memory");
        assert!(bytes_written <= crate::gameboy::rom::MAX_GAME_SIZE);

        memory
    }

    /// Writes a slice of data to the memory at the specified location.
    /// Mostly used for writing ROM data.
    pub fn write_slice(
        &mut self,
        location: usize,
        data: &[u8],
    ) -> anyhow::Result<usize, WriteError> {
        if location >= MEMORY_SIZE {
            return Err(WriteError::OutOfBounds);
        }

        let end = location
            .checked_add(data.len())
            .unwrap_or(MEMORY_SIZE)
            .min(MEMORY_SIZE);
        if end > MEMORY_SIZE {
            return Err(WriteError::SliceEndOutOfBounds);
        }

        // Writing at E000-FE00 writes at C000-DE00
        // If the location and the end are in the mirroring range, just call write_slice with
        // adjusted location.
        if MIRRORING_RANGE.contains(&location) && MIRRORING_RANGE.contains(&end) {
            return Self::write_slice(self, location - MIRRORING_OFFSET, data);
        }

        // If only a part of the slice is in the mirroring range, return an error. This is not
        // really expected to happen.
        if MIRRORING_RANGE.contains(&location) || MIRRORING_RANGE.contains(&end) {
            return Err(WriteError::SliceInMirroringRange);
        }

        let count = end.saturating_sub(location);
        self.ram[location..end].copy_from_slice(&data[..count]);
        Ok(count)
    }

    /// Writes a single byte to the memory at the specified location.
    pub fn write_byte(&mut self, location: usize, byte: u8) -> anyhow::Result<(), WriteError> {
        if location >= MEMORY_SIZE {
            return Err(WriteError::OutOfBounds);
        }

        // Writing at E000-FE00 writes at C000-DE00
        if MIRRORING_RANGE.contains(&location) {
            self.ram[location - MIRRORING_OFFSET] = byte;
        } else {
            self.ram[location] = byte;
        }

        Ok(())
    }
}

impl fmt::Debug for Memory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Bus")
    }
}

#[derive(Debug)]
pub enum WriteError {
    OutOfBounds,
    SliceEndOutOfBounds,
    SliceInMirroringRange,
}

impl fmt::Display for WriteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WriteError::OutOfBounds => write!(f, "Write operation out of bounds"),
            WriteError::SliceEndOutOfBounds => write!(f, "Slice end exceeds memory bounds"),
            WriteError::SliceInMirroringRange => write!(f, "Slice overlaps with mirroring range"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_slice_basic() {
        let mut memory = Memory::new();
        let data = [1, 2, 3, 4, 5];
        let bytes_written = memory.write_slice(0x0000, &data).unwrap();
        assert_eq!(bytes_written, data.len());
        assert_eq!(&memory.ram[0x0000..0x0005], &data);
    }

    #[test]
    fn test_write_slice_out_of_bounds() {
        let mut mem = Memory::new();
        let data = [1, 2, 3, 4];
        let start = MEMORY_SIZE - 2;
        let written = mem.write_slice(start, &data).unwrap();
        assert_eq!(written, 2);
        assert_eq!(&mem.ram[start..MEMORY_SIZE], &data[..2]);
    }

    #[test]
    fn test_write_slice_zero_length() {
        let mut mem = Memory::new();
        let data: [u8; 0] = [];
        let written = mem.write_slice(10, &data).unwrap();
        assert_eq!(written, 0);
    }

    #[test]
    fn test_write_slice_inside_mirroring() {
        let mut memory = Memory::new();
        let data = [1, 2, 3, 4, 5];
        // Writing at E000-FE00 writes at C000-DE00
        let bytes_written = memory.write_slice(0xE000, &data).unwrap();
        assert_eq!(bytes_written, data.len());
        assert_eq!(&memory.ram[0xC000..0xC005], &data);
        assert_eq!(&memory.ram[0xE000..0xE005], &[0; 5]); // Should not affect E000
    }

    #[test]
    fn test_write_slice_partial_overlap_mirroring() {
        let mut memory = Memory::new();
        let data = [1, 2, 3, 4, 5];
        // This should fail because it overlaps with the mirroring range
        // Writing at E000-FE00 writes at C000-DE00
        let result = memory.write_slice(0xFDFF, &data);
        assert!(matches!(result, Err(WriteError::SliceInMirroringRange)));
    }

    #[test]
    fn test_write_byte_basic() {
        let mut memory = Memory::new();
        let byte = 0x42;
        memory.write_byte(0x0000, byte).unwrap();
        assert_eq!(memory.ram[0x0000], byte);
    }

    #[test]
    fn test_write_byte_out_of_bounds() {
        let mut memory = Memory::new();
        let result = memory.write_byte(MEMORY_SIZE, 0x42);
        assert!(matches!(result, Err(WriteError::OutOfBounds)));
    }

    #[test]
    fn test_write_byte_e000_to_fe00() {
        let mut memory = Memory::new();
        let byte = 0x42;
        // Writing at E000-FE00 writes at C000-DE00
        memory.write_byte(0xE000, byte).unwrap();
        assert_eq!(memory.ram[0xC000], byte); // Should write to C000
        assert_eq!(memory.ram[0xE000], 0x00); // Should write to E000 as well
    }

    #[test]
    fn test_write_byte_c000_to_de00() {
        let mut memory = Memory::new();
        let byte = 0x42;
        // Writing at C000-DE00 writes at C000-DE00
        memory.write_byte(0xC000, byte).unwrap();
        assert_eq!(memory.ram[0xC000], byte); // Should write to C000
        assert_eq!(memory.ram[0xE000], 0x00); // Should not affect E000
    }
}
