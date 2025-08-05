//! Decoder for the Gameboy opcodes.
//!
//! The decoder will only decode the opcode and its operands if any into an `Instruction` struct.
//!

mod decoder;
mod operand;
mod reg_tables;

pub use decoder::Decoder;
pub use operand::Operand;
