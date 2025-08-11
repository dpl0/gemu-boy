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
pub(crate) enum R {
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
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0b000 => Some(R::B),
            0b001 => Some(R::C),
            0b010 => Some(R::D),
            0b011 => Some(R::E),
            0b100 => Some(R::H),
            0b101 => Some(R::L),
            0b110 => Some(R::IndirectHL),
            0b111 => Some(R::A),
            _ => None,
        }
    }
}

#[repr(u8)]
pub(crate) enum DD {
    BC = 0b00,
    DE = 0b01,
    HL = 0b10,
    SP = 0b11,
}

impl DD {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0b00 => Some(DD::BC),
            0b01 => Some(DD::DE),
            0b10 => Some(DD::HL),
            0b11 => Some(DD::SP),
            _ => None,
        }
    }
}

#[repr(u8)]
pub(crate) enum QQ {
    BC = 0b00,
    DE = 0b01,
    HL = 0b10,
    AF = 0b11,
}

impl QQ {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0b00 => Some(QQ::BC),
            0b01 => Some(QQ::DE),
            0b10 => Some(QQ::HL),
            0b11 => Some(QQ::AF),
            _ => None,
        }
    }
}

#[repr(u8)]
pub(crate) enum SS {
    BC = 0b00,
    DE = 0b01,
    HL = 0b10,
    SP = 0b11,
}

impl SS {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0b00 => Some(SS::BC),
            0b01 => Some(SS::DE),
            0b10 => Some(SS::HL),
            0b11 => Some(SS::SP),
            _ => None,
        }
    }
}
