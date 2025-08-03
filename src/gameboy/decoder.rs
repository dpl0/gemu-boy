//! Decoder for the Gameboy opcodes.
//!
//! The decoder will only decode the opcode and its operands if any into an `Instruction` struct.

/// Transform a 'Z N H C' string into a FlagEffects struct.
/// The input uses a legend for each flag:
///  - '1': Set 
///  - '0': Clear
///  - '-': No effect
///  - '?': Compute
macro_rules! flag_effects {
    ($z:tt $n:tt $h:tt $c:tt) => {
        FlagEffects { 
            z: flag_effect!($z),
            n: flag_effect!($n),
            h: flag_effect!($h),
            c: flag_effect!($c),
        }
    }
}

/// Transform a single flag effect into a FlagEffect enum.
macro_rules! flag_effect {
    (1) => { FlagEffect::Set };
    (0) => { FlagEffect::Clear };
    (-) => { FlagEffect::NoEffect };
    (?) => { FlagEffect::Compute };
    ($x:tt) => { compile_error!("Invalid flag effect (only 1, 0, -, or ?, can be used!") };
}


/// Decoder for the Gameboy opcodes.
#[derive(Debug, Default)]
pub struct Decoder;

impl Decoder {
    /// Creates a new instance of the Decoder.
    pub fn new() -> Self {
        Decoder {}
    }

    /// Fetches the instruction at the given program counter (PC) address from the RAM.
    pub fn fetch_instruction(&self, ram: &crate::gameboy::ram::Memory, pc: u16) -> anyhow::Result<Instruction> {
        let first_byte = ram.read_byte(pc);
        let second_byte = ram.read_byte(pc+1);
        let third_byte = ram.read_byte(pc+2);
        let bytes = [first_byte, second_byte, third_byte];

        Instruction::decode(&bytes)
    }
}


/// Represents a single instruction in the Gameboy CPU.
#[derive(Debug, Clone, Copy)]
pub struct Instruction {
    /// The opcode of the instruction.
    pub opcode: u8,
    /// The mnemonic of the instruction.
    pub mnemonic: Mnemonic,
    /// The operands of the instruction.
    pub operands: (Operand, Operand),
    /// The size of the instruction in bytes.
    pub size: u8,
    /// The number of cycles this instruction takes to execute.
    pub cycles: u8,
    /// The effects this instruction has on the CPU flags.
    pub flags_effects: FlagEffects,
}

/// These are the official register tables from the Game Boy manual.
mod reg_tables {
    use crate::gameboy::decoder::{Operand, Register16, Register8};

    #[repr(u8)]
    pub(crate) enum RegR {
        B = 0b000,
        C = 0b001,
        D = 0b010,
        E = 0b011,
        H = 0b100,
        L = 0b101,
        IndirectHL = 0b110,
        A = 0b111,
    }

    impl RegR {
        pub fn from_u8(value: u8) -> Option<Self> {
            match value {
                0b000 => Some(RegR::B),
                0b001 => Some(RegR::C),
                0b010 => Some(RegR::D),
                0b011 => Some(RegR::E),
                0b100 => Some(RegR::H),
                0b101 => Some(RegR::L),
                0b110 => Some(RegR::IndirectHL),
                0b111 => Some(RegR::A),
                _ => None,
            }
        }
    }

    // TODO: Prefer `From` instead of `Into`
    impl Into<Operand> for RegR {
        fn into(self) -> Operand {
            match self {
                RegR::B => Operand::Register8(Register8::B),
                RegR::C => Operand::Register8(Register8::C),
                RegR::D => Operand::Register8(Register8::D),
                RegR::E => Operand::Register8(Register8::E),
                RegR::H => Operand::Register8(Register8::H),
                RegR::L => Operand::Register8(Register8::L),
                RegR::IndirectHL => Operand::Indirect16(Register16::HL),
                RegR::A => Operand::Register8(Register8::A),
            }
        }
    }
}

impl Instruction {

    /// Creates a new instruction from a slice of bytes.
    pub fn decode(bytes: &[u8; 3]) -> anyhow::Result<Self> {
        match bytes[0] {
            bytes if bytes & 0b0100_0000 == 0b0100_0000 => {
                // LD 8b register a to 8b register b
                let operand_a: Operand = reg_tables::RegR::from_u8(bytes & 0b0000_0111).expect("Invalid register A").into();
                let operand_b: Operand = reg_tables::RegR::from_u8((bytes & 0b0011_1000) >> 3).expect("Invalid register B").into();

                Ok(Instruction {
                    opcode: bytes,
                    mnemonic: Mnemonic::Ld8,
                    operands: (operand_a, operand_b),
                    size: 1,
                    cycles: 1,
                    flags_effects: flag_effects!(- - - -),
                })
            },
            bytes if bytes == 0x00 => {
                // NOP instruction
                Ok(Instruction {
                    opcode: bytes,
                    mnemonic: Mnemonic::Nop,
                    operands: (Operand::None, Operand::None),
                    size: 1,
                    cycles: 4,
                    flags_effects: flag_effects!(- - - -),
                })
            },
            _ => todo!(),
        }
    }
}

/// Represents the opcode of an instruction.
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Mnemonic {
    Nop,
    Ld8,
}

/// Represents the operands of an instruction.
#[derive(Debug, Clone, Copy)]
pub enum Operand {
    /// No operand.
    None,
    /// The value in a 8-bit register. 
    Register8(Register8),
    /// The value in a 16-bit register. 
    Register16(Register16),
    /// An immediate 8 bit value.
    Immediate8(u8),
    /// An immediate 16 bit value.
    Immediate16(u16),
    /// An indirect 16-bit address stored in a 16-bit register.
    Indirect16(Register16),
    /// An immediate indirect value, which is a 16-bit address.
    IndirectImmediate16(u16)
}

/// 8-bit Register operands.
/// These are the registers that can be used by the CPU as operands when executing instructions.
#[derive(Debug, Clone, Copy)]
pub enum Register8 {
    A, B, C, D, E, H, L, F,
    SP, PC,
}

/// 16-bit Register operands.
/// These are the 16b registers that can be used by the CPU as operands when executing instructions.
#[derive(Debug, Clone, Copy)]
pub enum Register16 {
    AF, BC, DE, HL,
    SP, PC,
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


