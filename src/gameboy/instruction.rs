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
///  - 'Z|N|H|C': Compute
#[macro_export]
macro_rules! flag_effects {
    ($z:tt $n:tt $h:tt $c:tt) => {
        FlagEffects {
            z: flag_effect!(z, $z),
            n: flag_effect!(n, $n),
            h: flag_effect!(h, $h),
            c: flag_effect!(c, $c),
        }
    };
}

/// Transform a single flag effect into a [`FlagEffect`] enum member.
#[macro_export]
macro_rules! flag_effect {
    ($ignored:tt, 1) => {
        FlagEffect::Set
    };
    ($ignored:tt, 0) => {
        FlagEffect::Clear
    };
    ($ignored:tt, -) => {
        FlagEffect::NoEffect
    };
    (z, Z) => {
        FlagEffect::Compute
    };
    (n, N) => {
        FlagEffect::Compute
    };
    (h, H) => {
        FlagEffect::Compute
    };
    (c, C) => {
        FlagEffect::Compute
    };
    ($ignored:tt, $x:tt) => {
        compile_error!("Invalid flag effect (only 1, 0, -, or [Z | N | H | C], can be used!")
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flag_effect_macro() {
        assert!(matches!(flag_effect!(-, -), FlagEffect::NoEffect));
        assert!(matches!(flag_effect!(-, 0), FlagEffect::Clear));
        assert!(matches!(flag_effect!(-, 1), FlagEffect::Set));
        assert!(matches!(flag_effect!(z, Z), FlagEffect::Compute));
        assert!(matches!(flag_effect!(n, N), FlagEffect::Compute));
        assert!(matches!(flag_effect!(h, H), FlagEffect::Compute));
        assert!(matches!(flag_effect!(c, C), FlagEffect::Compute));
    }

    #[test]
    fn test_flag_effects_mixed() {
        assert!(matches!(
            flag_effects!(Z 0 1 -),
            FlagEffects {
                z: FlagEffect::Compute,
                n: FlagEffect::Clear,
                h: FlagEffect::Set,
                c: FlagEffect::NoEffect,
            },
        ));
    }
}
