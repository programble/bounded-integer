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
/// - `Rem<Self>`
/// - `Rem<Self::Repr>`
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
        bounded_integer_rem_self_impls!($ty);
        bounded_integer_rem_repr_impls!($ty);
    }
}

#[macro_use]
mod ops;
