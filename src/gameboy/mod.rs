/// This module contains the main GameBoy emulator structure and its components.
use camino::Utf8PathBuf;

mod cpu;
mod ram;
mod rom;
mod screen;

/// The main GameBoy structure that holds everything needed to run the emulator.
#[derive(Debug)]
pub struct GameBoy {
    pub rom_name: Utf8PathBuf,
    pub cpu: cpu::Cpu,
    pub ram: ram::Memory,
    pub screen: screen::Screen,
}

impl GameBoy {
    /// Creates a new instance of the GameBoy emulator with the given ROM file path.
    pub(crate) fn new(rom_name: Utf8PathBuf) -> anyhow::Result<GameBoy> {
        let rom = rom::Rom::new(&rom_name)?;

        Ok(GameBoy {
            rom_name,
            cpu: cpu::Cpu::new(),
            ram: ram::Memory::new_with_rom(rom),
            screen: screen::Screen::new(),
        })
    }
}
