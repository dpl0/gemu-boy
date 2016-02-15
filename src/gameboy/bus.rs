/*
 * The Bus will hold the needed stuff from the system.
 */

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
 * 
 */


use std::fmt;
use std::fs::File;
use std::io::Read;


#[derive(Default)]
pub struct Bus {
    ram: Vec<u8>,
    // ...
}

impl Bus {
    pub fn new(rom_name: &String) -> Bus {
        let ram = vec![0; 64 * 1024];

        let mut file = File::open(&rom_name).unwrap();
        let mut buf: Vec<u8> = Vec::new();
        file.read_to_end(&mut buf).unwrap();
        let buf = buf; // Shadow into an inmutable Copy.
        Bus { ram: ram }
    }
}

impl fmt::Debug for Bus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Bus")
    }
}
