//! Nibbles.

use std::cmp::Ordering;
use std::mem;

use BoundedInteger;

/// An unsigned nibble.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(missing_docs)]
#[repr(u8)]
pub enum UNibble { U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15 }

impl BoundedInteger for UNibble {
    type Repr = u8;

    fn from_repr(repr: u8) -> Option<Self> {
        if repr < 16 {
            Some(unsafe { mem::transmute(repr) })
        } else {
            None
        }
    }

    fn to_repr(self) -> u8 { self as u8 }

    fn min_value() -> Self { UNibble::U0 }
    fn max_value() -> Self { UNibble::U15 }
}

/// A signed nibble.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, Hash)]
#[allow(missing_docs)]
#[repr(u8)]
pub enum SNibble {
    N8 = 248,
    N7 = 249,
    N6 = 250,
    N5 = 251,
    N4 = 252,
    N3 = 253,
    N2 = 254,
    N1 = 255,
    U0 = 0,
    P1 = 1,
    P2 = 2,
    P3 = 3,
    P4 = 4,
    P5 = 5,
    P6 = 6,
    P7 = 7,
}

impl BoundedInteger for SNibble {
    type Repr = i8;

    fn from_repr(repr: i8) -> Option<Self> {
        if repr >= -8 && repr <= 7 {
            Some(unsafe { mem::transmute(repr) })
        } else {
            None
        }
    }

    fn to_repr(self) -> i8 { self as i8 }

    fn min_value() -> Self { SNibble::N8 }
    fn max_value() -> Self { SNibble::P7 }
}

impl PartialOrd for SNibble {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.to_repr().partial_cmp(&other.to_repr())
    }
}

/// A non-zero unsigned nibble.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(missing_docs)]
#[repr(u8)]
pub enum NZUNibble { U1 = 1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15 }

impl BoundedInteger for NZUNibble {
    type Repr = u8;

    fn from_repr(repr: u8) -> Option<Self> {
        if repr > 0 && repr < 16 {
            Some(unsafe { mem::transmute(repr) })
        } else {
            None
        }
    }

    fn to_repr(self) -> u8 { self as u8 }

    fn min_value() -> Self { NZUNibble::U1 }
    fn max_value() -> Self { NZUNibble::U15 }
}
