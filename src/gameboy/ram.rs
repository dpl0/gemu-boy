
/// This module implements the memory for Gemu-Boy.
///
/// The Game Boy has a 16-bit address bus, which is used to address ROM, RAM, and I/O.
///
///  This is a map of the address space:
///
///  0000 - 3FFF | 16 KiB ROM bank 00             | Cartridge, usually a fixed bank
///  4000 - 7FFF | 16 KiB ROM Bank 01–NN          | Cartridge, switchable bank
///  8000 - 9FFF | 8 KiB Video RAM (VRAM)         | In CGB mode, switchable bank 0/1
///  A000 - BFFF | 8 KiB External RAM             | Cartridge, switchable bank
///  C000 - CFFF | 4 KiB Work RAM (WRAM)          |
///  D000 - DFFF | 4 KiB Work RAM (WRAM)          | In CGB mode, switchable bank 1–7
///  E000 - FDFF | Echo RAM (mirror of C000–DDFF) | Forbidden by Nintendo.
///  FE00 - FE9F | Object attribute memory (OAM)  |
///  FEA0 - FEFF | Not Usable                     | Forbidden by Nintendo.
///  FF00 - FF7F | I/O Registers                  |
///  FF80 - FFFE | High RAM (HRAM)                |
///  FFFF - FFFF | Interrupt Enable register (IE) |
///
///  ## Echo RAM
///  Accesses to E000-FDFF go to C000-DDFF.
use std::fmt;

use crate::gameboy::rom::Rom;

/// Represents the size of the memory in bytes.
pub const MEMORY_SIZE: usize = 64 * 1024;

/// Represents the mirroring secton of the memory.
pub const MIRRORING_RANGE: std::ops::RangeInclusive<usize> = 0xE000..=0xFDFF;
/// Represents the offset used when accessing the mirrored section of the memory.
pub const MIRRORING_OFFSET: usize = 0xE000 - 0xC000;

/// Represents the whole memory space of the Game Boy.
pub(crate) struct Memory {
    ram: [u8; MEMORY_SIZE],
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
        const START_ROM_BANK: u16 = 0x0000;

        let mut memory = Self::new();
        let bytes_written = memory
            .write_slice(START_ROM_BANK, &rom.bytecode)
            .expect("Failed to write ROM data to memory");
        assert!(bytes_written <= crate::gameboy::rom::MAX_GAME_SIZE);

        memory
    }

    /// Helper method to calculate the memory location considering mirroring addresses.
    fn physical_address(&self, location: u16) -> usize {
        // Accesses to E000-FDFF go to C000-DDFF.
        if MIRRORING_RANGE.contains(&(location as usize)) {
            location as usize - MIRRORING_OFFSET
        } else {
            location as usize
        }
    }

    /// Reads a byte from the memory at the specified location.
    pub fn read(&self, location: u16) -> u8 {
        self.ram[self.physical_address(location)]
    }

    /// Writes a slice of data to the memory at the specified location.
    /// Mostly used for writing ROM data.
    pub fn write_slice(&mut self, location: u16, data: &[u8]) -> Result<usize, MemAccessError> {
        let location = location as usize;

        let Some(end) = location.checked_add(data.len()) else {
            return Err(MemAccessError::SliceEndOutOfBounds);
        };
        let end = end.min(MEMORY_SIZE);

        // Accesses to E000-FDFF go to C000-DDFF.
        // Writing exclusively in the mirroring range.
        if MIRRORING_RANGE.contains(&location) && MIRRORING_RANGE.contains(&end) {
            return Self::write_slice(self, (location - MIRRORING_OFFSET) as u16, data);
        }

        // Writing partially in the mirroring range is not supported.
        if MIRRORING_RANGE.contains(&location) || MIRRORING_RANGE.contains(&end) {
            return Err(MemAccessError::SliceInMirroringRange);
        }

        let count = end.saturating_sub(location);
        self.ram[location..end].copy_from_slice(&data[..count]);
        Ok(count)
    }

    /// Writes a single byte to the memory at the specified location.
    ///
    /// Accesses to E000-FDFF go to C000-DDFF.
    pub fn write_byte(&mut self, location: u16, byte: u8) {
        self.ram[self.physical_address(location)] = byte;
    }

    /// Reads directly from the memory, bypassing mirrorring logic.
    pub fn read_raw(&self, location: u16) -> u8 {
        self.ram[location as usize]
    }

    /// Writes byte directly from the memory, bypassing mirrorring logic.
    pub fn write_byte_raw(&mut self, location: u16, byte: u8) {
        self.ram[location as usize] = byte;
    }
}

impl fmt::Debug for Memory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Bus")
    }
}

/// Index into the memory.
/// This will panic if the index is out of bounds.
impl std::ops::Index<usize> for Memory {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= MEMORY_SIZE {
            panic!("Index out of bounds");
        }

        // Accesses to E000-FDFF go to C000-DDFF.
        &self.ram[self.physical_address(index as u16)]
    }
}

/// Mutably index into the memory.
/// This will panic if the index is out of bounds.
impl std::ops::IndexMut<usize> for Memory {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= MEMORY_SIZE {
            panic!("Index out of bounds");
        }

        // Accesses to E000-FDFF go to C000-DDFF.
        &mut self.ram[self.physical_address(index as u16)]
    }
}

#[derive(Debug)]
/// Represents errors that can occur when accessing memory.
pub enum MemAccessError {
    SliceEndOutOfBounds,
    SliceInMirroringRange,
}

impl fmt::Display for MemAccessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MemAccessError::SliceEndOutOfBounds => write!(f, "Slice end exceeds addressable space"),
            MemAccessError::SliceInMirroringRange => {
                write!(f, "Slice overlaps with mirroring range")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_non_mirroring() {
        let mut memory = Memory::new();
        memory.ram[0x0000] = 42; // Set a value to read
        let byte = memory.read(0x0000);
        assert_eq!(byte, 42);

        let byte = memory[MEMORY_SIZE - 1];
        assert_eq!(byte, 0); // Default value is 0
    }

    #[test]
    fn test_read_mirroring() {
        let mut memory = Memory::new();
        // Accesses to E000-FDFF go to C000-DDFF.
        // 1 byte before the mirroring range
        memory[0xDFFF] = 1;
        assert_eq!(memory[0xDFFF], 1);
        assert_eq!(memory[0xCFFF], 0); // Should not affect CFFF

        // Start of the mirroring range, will write to C000 instead.
        memory[0xE000] = 2; // mirorred
        assert_eq!(memory[0xE000], 2); // Reads from C000
        assert_eq!(memory[0xC000], 2);

        // Start+1 mirroring range, will write to C001 instead.
        memory[0xE001] = 3; // mirorred
        assert_eq!(memory[0xE001], 3); // Reads from C001
        assert_eq!(memory[0xC001], 3);

        // End-1 of mirroring range, will write to DDFE instead.
        memory[0xFDFE] = 4; // mirorred
        assert_eq!(memory[0xFDFE], 4); // Should read from DDFE
        assert_eq!(memory[0xDDFE], 4);

        // End of the mirroring range, will write to DDFF instead.
        memory[0xFDFF] = 5; // mirorred
        assert_eq!(memory[0xFDFF], 5); // Should read from DDFF
        assert_eq!(memory[0xDDFF], 5);

        // End+1 mirroring range, will write to FE00 instead.
        memory[0xFE00] = 6; // non-mirorred
        assert_eq!(memory[0xFE00], 6); // Should read from DE00
        assert_eq!(memory[0xDE00], 0);
    }

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
        let written = mem.write_slice(start as u16, &data).unwrap();
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
        // Accesses to E000-FDFF go to C000-DDFF.
        let bytes_written = memory.write_slice(0xE000, &data).unwrap();
        assert_eq!(bytes_written, data.len());
        assert_eq!(&memory.ram[0xC000..0xC005], &data);
        assert_eq!(&memory.ram[0xE000..0xE005], &[0; 5]); // Should not affect E000
    }

    #[test]
    fn test_write_slice_partial_overlap_mirroring() {
        let mut memory = Memory::new();
        let data = [1, 2, 3, 4, 5];
        // Accesses to E000-FDFF go to C000-DDFF.
        // This should fail because it partially overlaps with the mirroring range
        let result = memory.write_slice(0xFDFF, &data);
        assert!(matches!(result, Err(MemAccessError::SliceInMirroringRange)));
    }

    #[test]
    fn test_write_byte_basic() {
        let mut memory = Memory::new();
        let byte = 0x42;
        memory[0x0000] = byte;
        assert_eq!(memory.ram[0x0000], byte);
    }

    #[test]
    #[should_panic(expected = "Index out of bounds")]
    fn test_write_byte_out_of_bounds() {
        let mut memory = Memory::new();
        // Since write_byte cannot fail, we can only test IndexMut.
        memory[MEMORY_SIZE] = 0x42;
    }

    #[test]
    fn test_write_byte_e000_to_fe00() {
        let mut memory = Memory::new();
        let byte = 0x42;
        // Accesses to E000-FDFF go to C000-DDFF.
        memory.write_byte(0xE000, byte);
        assert_eq!(memory[0xC000], byte); // Should write to 0xC000
        assert_eq!(memory[0xE000], byte); // And to 0xE000
    }

    #[test]
    fn test_write_byte_c000_to_de00() {
        let mut memory = Memory::new();
        let byte = 0x42;
        // Accesses to E000-FDFF go to C000-DDFF.
        memory[0xC000] = byte;
        assert_eq!(memory.ram[0xC000], byte); // Should write to 0xC000
        assert_eq!(memory.ram[0xE000], 0x00); // Should not affect 0xE000
    }
}

