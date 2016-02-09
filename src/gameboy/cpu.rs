/*
 * Definition of the registers of the CPU.
 */

use super::bus;

#[derive (Default, Debug)]
pub struct Flag { // Stores all the flags. CORRECT It really is an u8.
   z: bool, // Zero, Non-Zero (set when the result of a math op is
            // zero, or two values are the same after CP.
   n: bool, // Set if the last math OP was a substraction.
   h: bool, // Half-Carry, set if a carry occurred from the lower
            // nibble in the last math op.
   c: bool, // Carry Flag, set if a carry occured from the last
            // math op, or if reg A is smaller when executing CP.
}

#[derive (Debug)]
pub struct RegisterFile {
   af: (u8,u8),
   bc: (u8, u8),
   de: (u8, u8),
   hl: (u8, u8),
   sp: u16, // Stack Pointer, used to keep track of the top of the stack.
            // 0xFFFE by default.
   pc: u16, // 0x100 by default.
   flag: Flag,
}

impl RegisterFile {
    fn new() -> RegisterFile {
        RegisterFile { sp: 0xfffe, pc: 0x100, .. RegisterFile::default() }
    }
}

// Start as after power-on at instantiation.
impl Default for RegisterFile {
    fn default() -> RegisterFile {
        RegisterFile { af: (0, 0), bc: (0, 0), de: (0, 0),
                       hl: (0, 0), sp: 0xfffe, pc: 0x100,
                       flag:Flag::default() }
    }
}


#[derive (Default, Debug)]
pub struct Cpu {
    pub regs: RegisterFile,
    pub bus: bus::Bus,
}

impl Cpu {
    pub fn new(rom_name: &String) -> Cpu {
        Cpu {
                regs: RegisterFile::new(),
                bus: bus::Bus::new(rom_name),
        }
    }
}
