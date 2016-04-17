//! Provides *bounded integers*, integer types which are restricted to a range of values. These
//! types are created by implementing the [`BoundedInteger`](trait.BoundedInteger.html) trait for
//! C-like enums.
//!
//! This crate provides macros for generating implementations of `BoundedInteger`, `Into`, and
//! arithmetic traits from `std::ops`. On nightly Rust, the [`bounded-integer-plugin`][plugin]
//! crate provides a procedural macro for generating enums with variants for a range.
//!
//! `bounded-integer` is on [Crates.io][crate] and [GitHub][github].
//!
//! [plugin]: https://cmcenroe.me/bounded-integer/bounded_integer_plugin
//! [crate]: https://crates.io/crates/bounded-integer
//! [github]: https://github.com/programble/bounded-integer
//!
//! # Examples
//!
//! ## Nightly
//!
//! ```ignore
//! #![feature(plugin)]
//! #![plugin(bounded_integer_plugin)]
//!
//! #[macro_use]
//! extern crate bounded_integer;
//!
//! bounded_integer! {
//!     /// Value that can fit in a nibble.
//!     #[derive(Debug)]
//!     pub enum Nibble: i8 { -8...7 }
//! }
//! # fn main() { }
//! ```
//!
//! ## Stable
//!
//! The above example is equivalent to the following.
//!
//! ```no_run
//! #[macro_use]
//! extern crate bounded_integer;
//!
//! /// Value that can fit in a nibble.
//! #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
//! #[repr(i8)]
//! pub enum Nibble {
//!     N8 = -8, N7, N6, N5, N4, N3, N2, N1, Z0, P1, P2, P3, P4, P5, P6, P7
//! }
//! bounded_integer_impls!(Nibble, i8, Nibble::N8, Nibble::P7);
//! # fn main() { }
//! ```

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

/// Bounded integers.
///
/// Provides conversion, minimum, maximum, checked and saturating arithmetic.
pub trait BoundedInteger: Copy + Eq + Ord {
    /// Integer representation.
    ///
    /// Should reflect the `#[repr(...)]` attribute of `Self`.
    type Repr: Repr;

    /// Converts from `Self::Repr` to `Self`.
    fn from_repr(repr: Self::Repr) -> Option<Self>;

    /// Converts from `Self` to `Self::Repr`.
    fn to_repr(self) -> Self::Repr;

    /// Returns the smallest value that can be represented as `Self`.
    fn min_value() -> Self;

    /// Returns the largest value that can be represented as `Self`.
    fn max_value() -> Self;

    // Checked arithmetic.

    /// Checked integer addition.
    fn checked_add(self, other: Self) -> Option<Self> {
        self.checked_add_repr(other.to_repr())
    }

    /// Checked integer subtraction.
    fn checked_sub(self, other: Self) -> Option<Self> {
        self.checked_sub_repr(other.to_repr())
    }

    /// Checked integer multiplication.
    fn checked_mul(self, other: Self) -> Option<Self> {
        self.checked_mul_repr(other.to_repr())
    }

    /// Checked integer division.
    fn checked_div(self, other: Self) -> Option<Self> {
        self.checked_div_repr(other.to_repr())
    }

    /// Checked integer remainder.
    fn checked_rem(self, other: Self) -> Option<Self> {
        self.checked_rem_repr(other.to_repr())
    }

    /// Checked integer negation.
    fn checked_neg(self) -> Option<Self> {
        self.to_repr().checked_neg().and_then(Self::from_repr)
    }

    // Checked arithmetic with `Self::Repr`.

    /// Checked integer addition with `Self::Repr`.
    fn checked_add_repr(self, other: Self::Repr) -> Option<Self> {
        self.to_repr().checked_add(other).and_then(Self::from_repr)
    }

    /// Checked integer subtraction with `Self::Repr`.
    fn checked_sub_repr(self, other: Self::Repr) -> Option<Self> {
        self.to_repr().checked_sub(other).and_then(Self::from_repr)
    }

    /// Checked integer multiplication with `Self::Repr`.
    fn checked_mul_repr(self, other: Self::Repr) -> Option<Self> {
        self.to_repr().checked_mul(other).and_then(Self::from_repr)
    }

    /// Checked integer division with `Self::Repr`.
    fn checked_div_repr(self, other: Self::Repr) -> Option<Self> {
        self.to_repr().checked_div(other).and_then(Self::from_repr)
    }

    /// Checked integer remainder with `Self::Repr`.
    fn checked_rem_repr(self, other: Self::Repr) -> Option<Self> {
        self.to_repr().checked_rem(other).and_then(Self::from_repr)
    }

    // Saturating arithmetic.

    /// Saturating integer addition.
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add_repr(other.to_repr())
    }

    /// Saturating integer subtraction.
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub_repr(other.to_repr())
    }

    /// Saturating integer multiplication.
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul_repr(other.to_repr())
    }

    // Saturating arithmetic with `Self::Repr`.

    /// Saturating integer addition with `Self::Repr`.
    fn saturating_add_repr(self, other: Self::Repr) -> Self {
        if other.is_negative() {
            self.checked_add_repr(other).unwrap_or(Self::min_value())
        } else {
            self.checked_add_repr(other).unwrap_or(Self::max_value())
        }
    }

    /// Saturating integer subtraction with `Self::Repr`.
    fn saturating_sub_repr(self, other: Self::Repr) -> Self {
        if other.is_negative() {
            self.checked_sub_repr(other).unwrap_or(Self::max_value())
        } else {
            self.checked_sub_repr(other).unwrap_or(Self::min_value())
        }
    }

    /// Saturating integer multiplication with `Self::Repr`.
    fn saturating_mul_repr(self, other: Self::Repr) -> Self {
        if self.to_repr().is_negative() == other.is_negative() {
            self.checked_mul_repr(other).unwrap_or(Self::max_value())
        } else {
            self.checked_mul_repr(other).unwrap_or(Self::min_value())
        }
    }
}
