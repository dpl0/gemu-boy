mod gameboy;

use camino::Utf8PathBuf;
use std::env;

fn main() -> anyhow::Result<()> {
    // TODO: Start using clap
    let rom_name = env::args().nth(1).unwrap();
    let rom_name: Utf8PathBuf = rom_name.into();

    let gameboy = gameboy::GameBoy::new(rom_name)?;
    println!("{gameboy:#?}");

    Ok(())
}
