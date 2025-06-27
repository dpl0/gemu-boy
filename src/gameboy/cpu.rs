//! Definition of the registers of the CPU.

/// Represents all flags as they'll be set by executing operations from the cpu.
/// These are actually the higher bits of the AF register!
#[derive(Default, Debug)]
pub struct Flags {
    // Zero, Non-Zero (set when the result of a math op is zero, or two values are the same after CP.
    z: bool,
    // Set if the last math OP was a subtraction.
    n: bool,
    /// Half-Carry, set if a carry occurred from the lower nibble in the last math op.
    h: bool,
    // Carry Flag, set if a carry occurred from the last math op, or if reg A is smaller when executing CP.
    c: bool,
}

#[derive(Debug)]
pub struct RegisterFile {
    af: u16,
    bc: u16,
    de: u16,
    hl: u16,
    sp: u16,
    pc: u16,
    flags: Flags,
}

impl RegisterFile {
    fn new() -> RegisterFile {
        RegisterFile::default()
    }
}

// Start as after power-on at instantiation.
impl Default for RegisterFile {
    fn default() -> RegisterFile {
        RegisterFile {
            af: 0,
            bc: 0,
            de: 0,
            hl: 0,
            sp: 0xfffe,
            pc: 0x100,
            // The flags are set to default, which is all false.
            // Note: The flags are the higher nibble of the AF register, so they are not stored in
            // the tuple.
            flags: Flags::default(),
        }
    }
}

#[derive(Default, Debug)]
pub struct Cpu {
    pub registers: RegisterFile,
}

impl Cpu {
    pub fn new() -> Self {
        Self::default()
    }
}
