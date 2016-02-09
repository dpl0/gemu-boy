/*
 * The GameBoy will be implemented in this module.
 */

mod cpu;
mod bus;
mod sound;

#[derive (Default, Debug)]
pub struct GameBoy {
    pub cpu: cpu::Cpu,
}

impl GameBoy {
    pub fn new(rom_name: &String) -> GameBoy {
        GameBoy {
            cpu: cpu::Cpu::new(rom_name),
        }
    }
}

