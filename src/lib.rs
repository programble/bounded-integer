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

use repr::Repr;

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

    /// Checked integer subtraction.
    fn checked_sub(self, other: Self) -> Option<Self> {
        self.checked_sub_repr(other.to_repr())
    }

    /// Checked integer subtraction with representation.
    fn checked_sub_repr(self, other: Self::Repr) -> Option<Self> {
        Self::from_repr(self.to_repr() + other)
    }

    /// Saturating integer addition.
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add_repr(other.to_repr())
    }

    /// Saturating integer addition with representation.
    fn saturating_add_repr(self, other: Self::Repr) -> Self {
        if other.is_negative() {
            self.checked_add_repr(other).unwrap_or(Self::min_value())
        } else {
            self.checked_add_repr(other).unwrap_or(Self::max_value())
        }
    }

    /// Saturating integer subtraction.
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub_repr(other.to_repr())
    }

    /// Saturating integer subtraction with representation.
    fn saturating_sub_repr(self, other: Self::Repr) -> Self {
        if other.is_negative() {
            self.checked_sub_repr(other).unwrap_or(Self::max_value())
        } else {
            self.checked_sub_repr(other).unwrap_or(Self::min_value())
        }
    }
}

pub mod repr;
pub mod bit;
pub mod trit;
