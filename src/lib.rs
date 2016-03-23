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

pub use repr::Repr;
mod repr;

#[macro_use]
mod macros;

/// Bounded integer.
pub trait BoundedInteger: Copy + Eq + Ord {
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
        self.to_repr().checked_add(other).and_then(Self::from_repr)
    }

    /// Checked integer subtraction.
    fn checked_sub(self, other: Self) -> Option<Self> {
        self.checked_sub_repr(other.to_repr())
    }

    /// Checked integer subtraction with representation.
    fn checked_sub_repr(self, other: Self::Repr) -> Option<Self> {
        self.to_repr().checked_sub(other).and_then(Self::from_repr)
    }

    /// Checked integer multiplication.
    fn checked_mul(self, other: Self) -> Option<Self> {
        self.checked_mul_repr(other.to_repr())
    }

    /// Checked integer multiplication with representation.
    fn checked_mul_repr(self, other: Self::Repr) -> Option<Self> {
        self.to_repr().checked_mul(other).and_then(Self::from_repr)
    }

    /// Checked integer division.
    fn checked_div(self, other: Self) -> Option<Self> {
        self.checked_div_repr(other.to_repr())
    }

    /// Checked integer division with representation.
    fn checked_div_repr(self, other: Self::Repr) -> Option<Self> {
        self.to_repr().checked_div(other).and_then(Self::from_repr)
    }

    /// Checked integer remainder.
    fn checked_rem(self, other: Self) -> Option<Self> {
        self.checked_rem_repr(other.to_repr())
    }

    /// Checked integer remainder with representation.
    fn checked_rem_repr(self, other: Self::Repr) -> Option<Self> {
        self.to_repr().checked_rem(other).and_then(Self::from_repr)
    }

    /// Checked integer negation.
    fn checked_neg(self) -> Option<Self> {
        self.to_repr().checked_neg().and_then(Self::from_repr)
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

    /// Saturating integer multiplication.
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul_repr(other.to_repr())
    }

    /// Saturating integer multiplication with representation.
    fn saturating_mul_repr(self, other: Self::Repr) -> Self {
        if self.to_repr().is_negative() == other.is_negative() {
            self.checked_mul_repr(other).unwrap_or(Self::max_value())
        } else {
            self.checked_mul_repr(other).unwrap_or(Self::min_value())
        }
    }
}
