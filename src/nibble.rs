//! Nibbles.

use std::cmp::Ordering;

use BoundedInteger;

/// An unsigned nibble.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(missing_docs)]
#[repr(u8)]
pub enum UNibble { U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15 }

bounded_integer_impl!(UNibble, u8, UNibble::U0 = 0, UNibble::U15 = 15);

/// A signed nibble.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, Hash)]
#[allow(missing_docs)]
#[repr(u8)]
pub enum SNibble { N8 = 248, N7, N6, N5, N4, N3, N2, N1, U0 = 0, P1, P2, P3, P4, P5, P6, P7 }

bounded_integer_impl!(SNibble, i8, SNibble::N8 = -8, SNibble::P7 = 7);

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

bounded_integer_impl!(NZUNibble, u8, NZUNibble::U1 = 1, NZUNibble::U15 = 15);
