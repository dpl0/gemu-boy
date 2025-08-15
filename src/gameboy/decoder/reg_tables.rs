//!
//! The official register tables from the Game Boy manual.
//!
//! This enums are used to decode the registers used in the CPU instructions as per the Nintendo
//! official documentation.
//!

/// The official register table for 8-bit registers.
///
/// The description actually has a missing value, which is the  [`IndirectHL`] register. This register is actually used to refer to the memory address pointed by the HL register, not the register itself.
#[repr(u8)]
pub enum R {
    B = 0b000,
    C = 0b001,
    D = 0b010,
    E = 0b011,
    H = 0b100,
    L = 0b101,
    IndirectHL = 0b110,
    A = 0b111,
}

impl R {
    pub const fn from_u8(value: u8) -> Option<Self> {
        match value {
            0b000 => Some(Self::B),
            0b001 => Some(Self::C),
            0b010 => Some(Self::D),
            0b011 => Some(Self::E),
            0b100 => Some(Self::H),
            0b101 => Some(Self::L),
            0b110 => Some(Self::IndirectHL),
            0b111 => Some(Self::A),
            _ => None,
        }
    }
}

#[repr(u8)]
pub enum DD {
    BC = 0b00,
    DE = 0b01,
    HL = 0b10,
    SP = 0b11,
}

impl DD {
    pub const fn from_u8(value: u8) -> Option<Self> {
        match value {
            0b00 => Some(Self::BC),
            0b01 => Some(Self::DE),
            0b10 => Some(Self::HL),
            0b11 => Some(Self::SP),
            _ => None,
        }
    }
}

#[repr(u8)]
pub enum QQ {
    BC = 0b00,
    DE = 0b01,
    HL = 0b10,
    AF = 0b11,
}

impl QQ {
    pub const fn from_u8(value: u8) -> Option<Self> {
        match value {
            0b00 => Some(Self::BC),
            0b01 => Some(Self::DE),
            0b10 => Some(Self::HL),
            0b11 => Some(Self::AF),
            _ => None,
        }
    }
}

#[repr(u8)]
pub enum SS {
    BC = 0b00,
    DE = 0b01,
    HL = 0b10,
    SP = 0b11,
}

impl SS {
    pub const fn from_u8(value: u8) -> Option<Self> {
        match value {
            0b00 => Some(Self::BC),
            0b01 => Some(Self::DE),
            0b10 => Some(Self::HL),
            0b11 => Some(Self::SP),
            _ => None,
        }
    }
}
