/*
 * The GameBoy will be implemented in this module.
 */

use camino::Utf8PathBuf;

mod cpu;
mod ram;
mod rom;
mod screen;

#[derive(Debug)]
pub struct GameBoy {
    pub cpu: cpu::Cpu,
    pub ram: ram::Memory,
    pub screen: screen::Screen,
}

impl GameBoy {
    pub fn new(rom_name: Utf8PathBuf) -> anyhow::Result<GameBoy> {
        let rom = rom::Rom::new(rom_name)?;

        Ok(GameBoy {
            cpu: cpu::Cpu::new(),
            ram: ram::Memory::new_with_rom(rom),
            screen: screen::Screen::new(),
        })
    }
}
