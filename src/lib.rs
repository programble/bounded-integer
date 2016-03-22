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

/// Implements `BoundedInteger` for an enum.
///
/// # Examples
///
/// ```
/// #[macro_use(bounded_integer_impl)]
/// extern crate bounded_integer;
///
/// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// #[repr(u8)]
/// enum TwoBit { U0, U1, U2, U3 }
/// bounded_integer_impl!(TwoBit, u8, TwoBit::U0, TwoBit::U3);
/// # fn main() { }
/// ```
#[macro_export]
macro_rules! bounded_integer_impl {
    ($ty:ty, $repr:ty, $min:path, $max:path) => {
        impl $crate::BoundedInteger for $ty {
            type Repr = $repr;

            #[allow(unused_comparisons)]
            fn from_repr(repr: $repr) -> Option<Self> {
                use std::mem;
                if repr >= $min as $repr && repr <= $max as $repr {
                    Some(unsafe { mem::transmute(repr) })
                } else {
                    None
                }
            }

            fn to_repr(self) -> $repr { self as $repr }

            fn min_value() -> Self { $min }
            fn max_value() -> Self { $max }
        }
    }
}

/// Implements `PartialOrd` for a `BoundedInteger` enum.
///
/// Only necessary for signed bounded integers. Otherwise, `PartialOrd` can be derived.
#[macro_export]
macro_rules! bounded_integer_partial_ord_impl {
    ($ty:ty) => {
        impl PartialOrd for $ty {
            fn partial_cmp(&self, other: &Self) -> Option<::std::cmp::Ordering> {
                use $crate::BoundedInteger;
                self.to_repr().partial_cmp(&other.to_repr())
            }
        }
    }
}

/// Implements `Into<Self::Repr>` for a `BoundedInteger` enum.
#[macro_export]
macro_rules! bounded_integer_into_repr_impl {
    ($ty:ty) => {
        impl Into<<$ty as $crate::BoundedInteger>::Repr> for $ty {
            fn into(self) -> <$ty as $crate::BoundedInteger>::Repr {
                use $crate::BoundedInteger;
                self.to_repr()
            }
        }
    }
}

/// Implements all bounded integer traits for an enum.
///
/// - `BoundedInteger`
/// - `Into<Self::Repr>`
/// - `Add<Self>`
/// - `Add<Self::Repr>`
/// - `Sub<Self>`
/// - `Sub<Self::Repr>`
/// - `Mul<Self>`
/// - `Mul<Self::Repr>`
/// - `Div<Self>`
/// - `Div<Self::Repr>`
///
/// # Examples
///
/// ```
/// #[macro_use]
/// extern crate bounded_integer;
///
/// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// #[repr(u8)]
/// enum TwoBit { U0, U1, U2, U3 }
/// bounded_integer_impls!(TwoBit, u8, TwoBit::U0, TwoBit::U3);
/// # fn main() { }
/// ```
#[macro_export]
macro_rules! bounded_integer_impls {
    ($ty:ty, $repr:ty, $min:path, $max:path) => {
        bounded_integer_impl!($ty, $repr, $min, $max);
        bounded_integer_into_repr_impl!($ty);
        bounded_integer_add_self_impls!($ty);
        bounded_integer_add_repr_impls!($ty);
        bounded_integer_sub_self_impls!($ty);
        bounded_integer_sub_repr_impls!($ty);
        bounded_integer_mul_self_impls!($ty);
        bounded_integer_mul_repr_impls!($ty);
        bounded_integer_div_self_impls!($ty);
        bounded_integer_div_repr_impls!($ty);
    }
}

#[macro_use]
mod ops_macros;
