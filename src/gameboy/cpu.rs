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
    // General purpose registers.
    a: u8,
    f: Flags,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    /// Stack pointer register.
    sp: u16,
    /// Program counter register.
    pc: u16,
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
            a: 0,
            f: Flags::default(),
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            sp: 0xfffe,
            pc: 0x100,
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
