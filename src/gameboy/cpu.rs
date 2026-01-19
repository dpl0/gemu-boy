//! Definition of the registers of the CPU.

use super::decoder::Decoder;
use super::ram::Memory;

/// Represents all flags as they'll be set by executing operations from the cpu.
/// These are actually the higher bits of the AF register!
// Allowing excessive bools because the flags are well represented as booleans.
#[derive(Default, Debug)]
#[allow(clippy::struct_excessive_bools)]
pub struct Flags {
    /// Zero, Non-Zero (set when the result of a math op is zero, or two values are the same after CP.
    z: bool,
    /// Set if the last math OP was a subtraction.
    n: bool,
    /// Half-Carry, set if a carry occurred from the lower nibble in the last math op.
    h: bool,
    /// Carry Flag, set if a carry occurred from the last math op, or if reg A is smaller when executing CP.
    c: bool,
}

#[derive(Debug)]
/// Contains all the registers of the CPU.
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
    fn new() -> Self {
        Self::default()
    }
}

// Start as after power-on at instantiation.
impl Default for RegisterFile {
    fn default() -> Self {
        Self {
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
    pub decoder: Decoder,
}

impl Cpu {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tick(&mut self, ram: &Memory) -> anyhow::Result<()> {
        let instruction = Decoder::fetch_instruction(ram, self.registers.pc)?;

        // Increment the program counter
        let (pc_incremented, overflow) = self
            .registers
            .pc
            .overflowing_add(u16::from(instruction.size));

        if overflow {
            log::warn!(
                "Program counter overflowed at PC: {:#04x}",
                self.registers.pc
            );
        }

        self.registers.pc = pc_incremented;

        // For now, just print the instruction for debugging purposes.
        println!("Executing: {:?}", &instruction);

        // Execute instruction
        // instruction.execute(ram)?;

        Ok(())
    }
}
