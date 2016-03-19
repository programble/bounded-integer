//! Bounded integers.

#![warn(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results,
    variant_size_differences,
)]

use std::hash::Hash;
use std::ops::Add;

/// Marker trait for integer representations.
pub trait Repr: Copy + Add<Self, Output=Self> { }

impl Repr for u8 { }
impl Repr for u16 { }
impl Repr for u32 { }
impl Repr for u64 { }

impl Repr for i8 { }
impl Repr for i16 { }
impl Repr for i32 { }
impl Repr for i64 { }

/// Bounded integer.
pub trait BoundedInteger: Copy + Eq + Ord + Hash {
    /// Integer representation.
    type Repr: Repr;

    /// Converts from representation to Self.
    fn from_repr(repr: Self::Repr) -> Option<Self>;

    /// Converts from Self to representation.
    fn to_repr(self) -> Self::Repr;

    /// Returns the smallest value that can be represented as Self.
    fn min_value() -> Self;

    /// Returns the largest value that can be represented as Self.
    fn max_value() -> Self;

    /// Checked integer addition.
    fn checked_add(self, other: Self) -> Option<Self> {
        self.checked_add_repr(other.to_repr())
    }

    /// Checked integer addition with representation.
    fn checked_add_repr(self, other: Self::Repr) -> Option<Self> {
        Self::from_repr(self.to_repr() + other)
    }
}

pub mod bit;
pub mod trit;
