mod gameboy;

use std::env;

// pub fn create_cpu() {
//     let R: cpu::RegisterFile = cpu::RegisterFile::new();
//     let M: memory::MemoryTable = memory::MemoryTable::new();
//     struct CPU {
//         registers: R,
//         memory: M,
//     }
// }

fn main() {
    let rom_name = env::args().nth(1).unwrap();

    let gameboy = gameboy::GameBoy::new(&rom_name);
    println!("{:#?}", gameboy);
}
