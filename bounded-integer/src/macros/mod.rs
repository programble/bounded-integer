/// Implements [`BoundedInteger`](trait.BoundedInteger.html) for a C-like enum with contiguous
/// variants.
///
/// Uses `std::mem::transmute` to implement `BoundedInteger::from_repr` and `as` to implement
/// `BoundedInteger::to_repr`.
///
/// # Examples
///
/// ```
/// # #[macro_use]
/// # extern crate bounded_integer;
/// #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// #[repr(u8)]
/// enum TwoBit { Z0, P1, P2, P3 }
/// bounded_integer_impl!(TwoBit, u8, TwoBit::Z0, TwoBit::P3);
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

/// Implements `Into<Self::Repr>` for a [`BoundedInteger`](trait.BoundedInteger.html).
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

/// Implements all bounded integer traits for a C-like enum with contiguous variants.
///
/// - [`BoundedInteger`](trait.BoundedInteger.html)
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
/// - `Neg`
///
/// # Examples
///
/// ```
/// # #[macro_use]
/// # extern crate bounded_integer;
/// #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// #[repr(u8)]
/// enum TwoBit { Z0, P1, P2, P3 }
/// bounded_integer_impls!(TwoBit, u8, TwoBit::Z0, TwoBit::P3);
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
        bounded_integer_neg_impls!($ty);
    }
}

#[macro_use]
mod ops;
