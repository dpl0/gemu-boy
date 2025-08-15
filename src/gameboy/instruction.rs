use super::decoder::Operand;

/// Represents a single instruction in the Gameboy CPU.
#[derive(Debug, Clone, Copy)]
pub struct Instruction {
    /// The opcode of the instruction.
    pub opcode: u8,
    /// The mnemonic of the instruction.
    pub mnemonic: Mnemonic,
    /// The source operand of the instruction.
    pub source: Operand,
    /// The destination operand of the instruction.
    pub destination: Operand,
    /// The size of the instruction in bytes.
    pub size: u8,
    /// The number of cycles this instruction takes to execute.
    pub cycles: u8,
    /// The effects this instruction has on the CPU flags.
    pub flags_effects: FlagEffects,
}

/// Represents the opcode of an instruction.
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Mnemonic {
    Nop,
    Ld8,
}

/// Transform a 'Z N H C' string into a [`FlagEffects`] struct.
/// The input uses a legend for each flag:
///  - '1': Set
///  - '0': Clear
///  - '-': No effect
///  - '?': Compute
#[macro_export]
macro_rules! flag_effects {
    ($z:tt $n:tt $h:tt $c:tt) => {
        FlagEffects {
            z: flag_effect!($z),
            n: flag_effect!($n),
            h: flag_effect!($h),
            c: flag_effect!($c),
        }
    };
}

/// Transform a single flag effect into a [`FlagEffect`] enum.
#[macro_export]
macro_rules! flag_effect {
    (1) => {
        FlagEffect::Set
    };
    (0) => {
        FlagEffect::Clear
    };
    (-) => {
        FlagEffect::NoEffect
    };
    (?) => {
        FlagEffect::Compute
    };
    ($x:tt) => {
        compile_error!("Invalid flag effect (only 1, 0, -, or ?, can be used!")
    };
}

/// Represents the flags that may be affected by instructions.
#[derive(Debug, Clone, Copy)]
pub struct FlagEffects {
    pub z: FlagEffect,
    pub n: FlagEffect,
    pub h: FlagEffect,
    pub c: FlagEffect,
}

/// How a flag may be affected by an instruction.
#[derive(Debug, Clone, Copy)]
pub enum FlagEffect {
    /// The flag will be set to 1.
    Set,
    /// The flag will be set to 0.
    Clear,
    /// The flag will not be affected by the instruction execution.
    NoEffect,
    /// The flag will be set depending on the instruction execution.
    Compute,
}
