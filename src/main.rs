mod cartridge;
mod cpu;
mod memory;

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

    let game = cartridge::Cartridge::new(rom_name);
    let mem = memory::build_memory(&game);
    // let cpu = cpu::Cpu;
    // println("{:?}", cpu);
}
