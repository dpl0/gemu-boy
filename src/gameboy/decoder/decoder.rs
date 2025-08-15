//! Decoder for the Gameboy opcodes.

use bitmatch::bitmatch;

use super::super::instruction::{FlagEffect, FlagEffects, Instruction, Mnemonic};
use super::operand::{Operand, Reg16};

use crate::{flag_effect, flag_effects};

/// Decoder for the Gameboy opcodes.
#[derive(Debug, Default)]
pub struct Decoder;

impl Decoder {
    /// Creates a new instance of the Decoder.
    pub const fn new() -> Self {
        Self {}
    }

    /// Fetches the instruction at the given program counter (PC) address from the RAM.
    pub fn fetch_instruction(
        &self,
        ram: &crate::gameboy::ram::Memory,
        pc: u16,
    ) -> anyhow::Result<Instruction> {
        let first_byte = ram.read_byte(pc);
        let second_byte = ram.read_byte(pc + 1);
        let third_byte = ram.read_byte(pc + 2);
        let bytes = [first_byte, second_byte, third_byte];

        self.decode(bytes)
    }

    /// Decodes the given bytes into an `Instruction`.
    /// This method expects a slice of 3 bytes, which is the maximum size of an instruction in the
    /// Gameboy CPU.
    // This `allow` config is to avoid the clippy warning for the generate code from `bitmatch`
    #[allow(clippy::verbose_bit_mask)]
    #[bitmatch]
    pub fn decode(&self, bytes: [u8; 3]) -> anyhow::Result<Instruction> {
        #[bitmatch]
        match bytes[0] {
            // This decoding is based on the official Game Boy manual by Nintendo, following the
            // instruction set order.
            // 8-bit Transfer and I/O Instructions
            "01aaabbb" => {
                // LD r <- r'
                let source = Operand::from_u8_as_r(a);
                let destination = Operand::from_u8_as_r(b);
                // If we have Indirect HL this is a special case
                match (source, destination) {
                    (Operand::Ind16(Reg16::HL), Operand::Ind16(Reg16::HL)) => {
                        // LD (HL) <- n
                        // This only happens when both source and dest are (HL)
                        Ok(Instruction {
                            opcode: bytes[0],
                            mnemonic: Mnemonic::Nop,
                            source: Operand::Imm8(bytes[1]),
                            destination,
                            size: 2,
                            cycles: 3,
                            flags_effects: crate::flag_effects!(- - - -),
                        })
                    }
                    (Operand::Ind16(Reg16::HL), _) => {
                        // LD r <- (HL)
                        Ok(Instruction {
                            opcode: bytes[0],
                            mnemonic: Mnemonic::Ld8,
                            source,
                            destination,
                            size: 1,
                            cycles: 2,
                            flags_effects: flag_effects!(- - - -),
                        })
                    }
                    (_, Operand::Ind16(Reg16::HL)) => {
                        // LD (HL) <- r
                        Ok(Instruction {
                            opcode: bytes[0],
                            mnemonic: Mnemonic::Ld8,
                            source,
                            destination,
                            size: 1,
                            cycles: 2,
                            flags_effects: flag_effects!(- - - -),
                        })
                    }
                    _ => {
                        // LD r <- r'
                        Ok(Instruction {
                            opcode: bytes[0],
                            mnemonic: Mnemonic::Ld8,
                            source,
                            destination,
                            size: 1,
                            cycles: 1,
                            flags_effects: flag_effects!(- - - -),
                        })
                    }
                }
            }
            "00aaa110" => {
                // LD r <- n
                let destination = Operand::from_u8_as_r(a);
                let immediate = Operand::Imm8(bytes[1]);
                Ok(Instruction {
                    opcode: bytes[0],
                    mnemonic: Mnemonic::Ld8,
                    source: immediate,
                    destination,
                    size: 2,
                    cycles: 2,
                    flags_effects: flag_effects!(- - - -),
                })
            }
            // Already implemented at "01aaabbb":
            // "01aaa110"
            // "01110aaa"
            // "01110110"
            "00000000" => {
                // NOP instruction
                Ok(Instruction {
                    opcode: bytes[0],
                    mnemonic: Mnemonic::Nop,
                    source: Operand::None,
                    destination: Operand::None,
                    size: 1,
                    cycles: 4,
                    flags_effects: flag_effects!(- - - -),
                })
            }
            _ => todo!(),
        }
    }
}
