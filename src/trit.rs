//! Trits.

/// An unsigned trit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(missing_docs)]
#[repr(u8)]
pub enum UTrit { U0, U1, U2 }

bounded_integer_impl!(UTrit, u8, UTrit::U0, UTrit::U2);

/// A signed (balanced) trit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, Hash)]
#[allow(missing_docs)]
#[repr(u8)]
pub enum STrit {
    N1 = 255,
    U0 = 0,
    P1 = 1,
}

bounded_integer_impl!(STrit, i8, STrit::N1, STrit::P1);
bounded_integer_partial_ord_impl!(STrit);
