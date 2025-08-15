use camino::Utf8PathBuf;

use std::fs::File;
use std::io::Read;

pub const MAX_GAME_SIZE: usize = 32 * 1024;

/// Represents a loaded rom from a file.
///
/// This struct assumes the rom will be loaded directly to memory.
///
pub struct Rom {
    pub bytecode: Box<[u8]>,
}

impl Rom {
    /// Creates a Rom struct with the passed path.
    pub fn new(rom_path: &Utf8PathBuf) -> anyhow::Result<Self> {
        let mut bytecode = vec![0; MAX_GAME_SIZE].into_boxed_slice();

        let mut file = File::open(rom_path)?;
        let bytes_read = file.read(&mut bytecode)?;
        assert!(bytes_read <= MAX_GAME_SIZE);

        Ok(Self { bytecode })
    }
}
