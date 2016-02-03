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
 * NOTE: b = bit, B = byte
 *
 *
 * Writing at E000-FE00 writes at C000-DE00
 * Writing at C000-DE00 writes at C000-DE00
 */

// CORRECT Memory should be filled with random data (except cartridge).
struct Memory {
   a:u32,
}

pub fn build_memory(C: &Cartridge) -> Memory {
}
