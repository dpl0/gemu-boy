//! This module defines the `Address` type, which is a wrapper around a 16-bit unsigned integer.
//! It is used to represent memory addresses in the Gameboy architecture.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub struct Address(u16);

impl Address {
    /// Creates a new `Address` from a 16-bit unsigned integer.
    pub const fn new(addr: u16) -> Self {
        Self(addr)
    }

    /// Returns the underlying address as a 16-bit unsigned integer.
    pub const fn as_u16(self) -> u16 {
        self.0
    }
}

impl From<u16> for Address {
    fn from(addr: u16) -> Self {
        Self::new(addr)
    }
}

impl From<Address> for u16 {
    fn from(addr: Address) -> Self {
        addr.as_u16()
    }
}

impl From<Address> for usize {
    fn from(addr: Address) -> Self {
        #![allow(clippy::use_self)]
        addr.as_u16() as usize
    }
}
