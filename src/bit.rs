//! Bits.

use BoundedInteger;

/// A bit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(missing_docs)]
#[repr(u8)]
pub enum Bit { U0, U1 }

impl BoundedInteger for Bit {
    type Repr = u8;

    fn from_repr(repr: u8) -> Option<Self> {
        match repr {
            0 => Some(Bit::U0),
            1 => Some(Bit::U1),
            _ => None,
        }
    }

    fn to_repr(self) -> u8 { self as u8 }

    fn min_value() -> Self { Bit::U0 }
    fn max_value() -> Self { Bit::U1 }
}
