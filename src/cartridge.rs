use std::fs::File;
use std::io::Read;

/*
 * Read and get worlds of the ROM/Cartridge word by word.
 */

#[derive (Default)]
pub struct Cartridge {
    data :Vec<u8>,
}

impl Cartridge {
    pub fn new(rom_name: String) -> Cartridge {
        let mut file = File::open(&rom_name).unwrap();
        let mut buf: Vec<u8> = Vec::new();
        file.read_to_end(&mut buf).unwrap();
        let buf = buf; // Shadow into an inmutable Copy.
        Cartridge { data: buf, }
    }

    pub fn read_word() -> u8 {
        return 0;
    }
}
