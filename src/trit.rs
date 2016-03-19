//! Trits.

use BoundedInteger;

/// An unsigned trit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(missing_docs)]
#[repr(u8)]
pub enum UTrit { U0, U1, U2 }

impl BoundedInteger for UTrit {
    type Repr = u8;

    fn from_repr(repr: u8) -> Option<Self> {
        match repr {
            0 => Some(UTrit::U0),
            1 => Some(UTrit::U1),
            2 => Some(UTrit::U2),
            _ => None,
        }
    }

    fn to_repr(self) -> u8 { self as u8 }

    fn min_value() -> Self { UTrit::U0 }
    fn max_value() -> Self { UTrit::U2 }
}

/// A signed (balanced) trit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(missing_docs)]
#[repr(u8)]
pub enum STrit {
    N1 = 255,
    U0 = 0,
    P1 = 1,
}

impl BoundedInteger for STrit {
    type Repr = i8;

    fn from_repr(repr: i8) -> Option<Self> {
        match repr {
            -1 => Some(STrit::N1),
            0 => Some(STrit::U0),
            1 => Some(STrit::P1),
            _ => None,
        }
    }

    fn to_repr(self) -> i8 { self as i8 }

    fn min_value() -> Self { STrit::N1 }
    fn max_value() -> Self { STrit::P1 }
}
