/// This module contains the main [`GameBoy`] emulator structure and its components.
use camino::Utf8PathBuf;

mod cpu;
mod decoder;
mod instruction;
mod ram;
mod rom;
mod screen;

/// The main [`GameBoy`] structure that holds everything needed to run the emulator.
#[derive(Debug)]
pub struct GameBoy {
    pub rom_name: Utf8PathBuf,
    pub cpu: cpu::Cpu,
    pub ram: ram::Memory,
    pub screen: screen::Screen,
    pub ticks: u64,
}

impl GameBoy {
    /// Creates a new instance of the [`GameBoy`] emulator with the given ROM file path.
    pub(crate) fn new(rom_name: Utf8PathBuf) -> anyhow::Result<Self> {
        let rom = rom::Rom::new(&rom_name)?;

        Ok(Self {
            rom_name,
            cpu: cpu::Cpu::new(),
            ram: ram::Memory::new_with_rom(&rom),
            screen: screen::Screen::new(),
            ticks: 0,
        })
    }

    /// Initialize the needed stuff for the GB.
    pub(crate) fn initialize(&self) -> anyhow::Result<()> {
        todo!()
    }

    /// Runs the [`GameBoy`], starts the CPU and begins executing instructions.
    pub(crate) fn run(&mut self) -> anyhow::Result<()> {
        loop {
            // The CPU will take care of everything, like fetching instruction, decoding, and
            // executing.
            self.cpu.tick(&self.ram)?;

            // Update the screen with the current state of the RAM.
            self.screen.update(&self.ram);

            self.ticks += 1;

            // TODO: Handle input, timing, and break condition if needed.
        }
    }
}
