//! Bits.

/// A bit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(missing_docs)]
#[repr(u8)]
pub enum Bit { U0, U1 }
bounded_integer_impl!(Bit, u8, Bit::U0, Bit::U1);
