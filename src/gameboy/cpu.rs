/*
 * Definition of the different parts of the CPU, as well as its control.
 */


#[derive (Debug)]
struct Flag { // Stores all the flags. CORRECT It really is an u8.
   z: bool, // Zero, Non-Zero (set when the result of a math op is
            // zero, or two values are the same after CP.
   n: bool, // Set if the last math OP was a substraction.
   h: bool, // Half-Carry, set if a carry occurred from the lower
            // nibble in the last math op.
   c: bool, // Carry Flag, set if a carry occured from the last
            // math op, or if reg A is smaller when executing CP.
}

impl Default for Flag {
    fn default() -> Flag {
        Flag { z: false, n: false,
               h: false, c: false, }
    }
}

#[derive (Debug)]
struct RegisterFile {
   af: (u8,u8),
   bc: (u8, u8),
   de: (u8, u8),
   hl: (u8, u8),
   sp: u16, // Stack Pointer, used to keep track of the top of the stack.
            // 0xFFFE by default.
   pc: u16, // 0x100 by default.
   flag: Flag,
}

// Start as after power-on at creation.
impl Default for RegisterFile {
    fn default() -> RegisterFile {
        RegisterFile { af: (0, 0), bc: (0, 0), de: (0, 0),
                       hl: (0, 0), sp: 0xfffe, pc: 0x100,
                       flag:Flag::default() }
    }
}
