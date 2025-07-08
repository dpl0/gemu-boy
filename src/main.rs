#![allow(dead_code)]

mod gameboy;

use camino::Utf8PathBuf;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path the ROM file to load.
    rom_name: Utf8PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let mut gameboy = gameboy::GameBoy::new(args.rom_name)?;
    gameboy.initialize()?;
    println!("GameBoy initialized with ROM: {}", gameboy.rom_name);
    gameboy.run()?;

    Ok(())
}
