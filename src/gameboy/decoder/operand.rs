use super::reg_tables::{DD, QQ, R, SS};

/// 8-bit Register operands.
/// These are the registers that can be used by the CPU as operands when executing instructions.
#[derive(Debug, Clone, Copy)]
pub enum Reg8 {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    F,
}

/// 16-bit Register operands.
/// These are the 16b registers that can be used by the CPU as operands when executing instructions.
#[derive(Debug, Clone, Copy)]
pub enum Reg16 {
    AF,
    BC,
    DE,
    HL,
    SP,
    PC,
}

/// Represents the operands of an instruction.
#[derive(Debug, Clone, Copy)]
pub enum Operand {
    /// No operand.
    None,
    /// The value in a 8-bit register.
    Reg8(Reg8),
    /// The value in a 16-bit register.
    Reg16(Reg16),
    /// An immediate 8 bit value.
    Imm8(u8),
    /// An immediate 16 bit value.
    Imm16(u16),
    /// An indirect 16-bit address stored in a 16-bit register.
    Ind16(Reg16),
}

impl Operand {
    pub fn from_u8_as_r(reg: u8) -> Self {
        R::from_u8(reg).expect("Invalid register value").into()
    }

    pub fn from_u8_as_dd(reg: u8) -> Self {
        DD::from_u8(reg).expect("Invalid register value").into()
    }

    pub fn from_u8_as_qq(reg: u8) -> Self {
        QQ::from_u8(reg).expect("Invalid register value").into()
    }

    pub fn from_u8_as_ss(reg: u8) -> Self {
        SS::from_u8(reg).expect("Invalid register value").into()
    }
}

impl From<R> for Operand {
    fn from(reg: R) -> Self {
        match reg {
            R::B => Self::Reg8(Reg8::B),
            R::C => Self::Reg8(Reg8::C),
            R::D => Self::Reg8(Reg8::D),
            R::E => Self::Reg8(Reg8::E),
            R::H => Self::Reg8(Reg8::H),
            R::L => Self::Reg8(Reg8::L),
            R::IndirectHL => Self::Ind16(Reg16::HL),
            R::A => Self::Reg8(Reg8::A),
        }
    }
}

impl From<DD> for Operand {
    fn from(dd: DD) -> Self {
        match dd {
            DD::BC => Self::Reg16(Reg16::BC),
            DD::DE => Self::Reg16(Reg16::DE),
            DD::HL => Self::Reg16(Reg16::HL),
            DD::SP => Self::Reg16(Reg16::SP),
        }
    }
}

impl From<QQ> for Operand {
    fn from(qq: QQ) -> Self {
        match qq {
            QQ::BC => Self::Reg16(Reg16::BC),
            QQ::DE => Self::Reg16(Reg16::DE),
            QQ::HL => Self::Reg16(Reg16::HL),
            QQ::AF => Self::Reg16(Reg16::AF),
        }
    }
}

impl From<SS> for Operand {
    fn from(ss: SS) -> Self {
        match ss {
            SS::BC => Self::Reg16(Reg16::BC),
            SS::DE => Self::Reg16(Reg16::DE),
            SS::HL => Self::Reg16(Reg16::HL),
            SS::SP => Self::Reg16(Reg16::SP),
        }
    }
}
