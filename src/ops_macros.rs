/// Implements `std::ops::Add` for a `BoundedInteger` enum with `Self`.
///
/// Implements the following combinations. The `Output` is always `Self`.
///
/// - `Self + Self`
/// - `Self + &Self`
/// - `&Self + Self`
/// - `&Self + &Self`
///
/// The implementations always panic on overflow.
#[macro_export]
macro_rules! bounded_integer_add_self_impls {
    ($ty:ty) => {
        impl ::std::ops::Add<$ty> for $ty {
            type Output = Self;
            fn add(self, rhs: Self) -> Self {
                use $crate::BoundedInteger;
                self.checked_add(rhs).expect("arithmetic operation overflowed")
            }
        }

        impl<'a> ::std::ops::Add<&'a $ty> for $ty {
            type Output = Self;
            fn add(self, rhs: &Self) -> Self { self + *rhs }
        }

        impl<'a> ::std::ops::Add<$ty> for &'a $ty {
            type Output = $ty;
            fn add(self, rhs: $ty) -> $ty { *self + rhs }
        }

        impl<'a, 'b> ::std::ops::Add<&'b $ty> for &'a $ty {
            type Output = $ty;
            fn add(self, rhs: &$ty) -> $ty { *self + *rhs }
        }
    }
}

/// Implements `std::ops::Add` for a `BoundedInteger` enum with `Self::Repr`.
///
/// Implements the following combinations. The `Output` is always `Self`.
///
/// - `Self + Self::Repr`
/// - `Self + &Self::Repr`
/// - `&Self + Self::Repr`
/// - `&Self + &Self::Repr`
///
/// The implementations always panic on overflow.
#[macro_export]
macro_rules! bounded_integer_add_repr_impls {
    ($ty:ty) => {
        impl ::std::ops::Add<<$ty as $crate::BoundedInteger>::Repr> for $ty {
            type Output = Self;
            fn add(self, rhs: <$ty as $crate::BoundedInteger>::Repr) -> Self {
                use $crate::BoundedInteger;
                self.checked_add_repr(rhs).expect("arithmetic operation overflowed")
            }
        }

        impl<'a> ::std::ops::Add<&'a <$ty as $crate::BoundedInteger>::Repr> for $ty {
            type Output = Self;
            fn add(self, rhs: &'a <$ty as $crate::BoundedInteger>::Repr) -> Self { self + *rhs }
        }

        impl<'a> ::std::ops::Add<<$ty as $crate::BoundedInteger>::Repr> for &'a $ty {
            type Output = $ty;
            fn add(self, rhs: <$ty as $crate::BoundedInteger>::Repr) -> $ty { *self + rhs }
        }

        impl<'a, 'b> ::std::ops::Add<&'b <$ty as $crate::BoundedInteger>::Repr> for &'a $ty {
            type Output = $ty;
            fn add(self, rhs: &<$ty as $crate::BoundedInteger>::Repr) -> $ty { *self + *rhs }
        }
    }
}

/// Implements `std::ops::Sub` for a `BoundedInteger` enum with `Self`.
///
/// Implements the following combinations. The `Output` is always `Self`.
///
/// - `Self - Self`
/// - `Self - &Self`
/// - `&Self - Self`
/// - `&Self - &Self`
///
/// The implementations always panic on overflow.
#[macro_export]
macro_rules! bounded_integer_sub_self_impls {
    ($ty:ty) => {
        impl ::std::ops::Sub<$ty> for $ty {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self {
                use $crate::BoundedInteger;
                self.checked_sub(rhs).expect("arithmetic operation overflowed")
            }
        }

        impl<'a> ::std::ops::Sub<&'a $ty> for $ty {
            type Output = Self;
            fn sub(self, rhs: &Self) -> Self { self - *rhs }
        }

        impl<'a> ::std::ops::Sub<$ty> for &'a $ty {
            type Output = $ty;
            fn sub(self, rhs: $ty) -> $ty { *self - rhs }
        }

        impl<'a, 'b> ::std::ops::Sub<&'b $ty> for &'a $ty {
            type Output = $ty;
            fn sub(self, rhs: &$ty) -> $ty { *self - *rhs }
        }
    }
}

/// Implements `std::ops::Sub` for a `BoundedInteger` enum with `Self::Repr`.
///
/// Implements the following combinations. The `Output` is always `Self`.
///
/// - `Self - Self::Repr`
/// - `Self - &Self::Repr`
/// - `&Self - Self::Repr`
/// - `&Self - &Self::Repr`
///
/// The implementations always panic on overflow.
#[macro_export]
macro_rules! bounded_integer_sub_repr_impls {
    ($ty:ty) => {
        impl ::std::ops::Sub<<$ty as $crate::BoundedInteger>::Repr> for $ty {
            type Output = Self;
            fn sub(self, rhs: <$ty as $crate::BoundedInteger>::Repr) -> Self {
                use $crate::BoundedInteger;
                self.checked_sub_repr(rhs).expect("arithmetic operation overflowed")
            }
        }

        impl<'a> ::std::ops::Sub<&'a <$ty as $crate::BoundedInteger>::Repr> for $ty {
            type Output = Self;
            fn sub(self, rhs: &'a <$ty as $crate::BoundedInteger>::Repr) -> Self { self - *rhs }
        }

        impl<'a> ::std::ops::Sub<<$ty as $crate::BoundedInteger>::Repr> for &'a $ty {
            type Output = $ty;
            fn sub(self, rhs: <$ty as $crate::BoundedInteger>::Repr) -> $ty { *self - rhs }
        }

        impl<'a, 'b> ::std::ops::Sub<&'b <$ty as $crate::BoundedInteger>::Repr> for &'a $ty {
            type Output = $ty;
            fn sub(self, rhs: &<$ty as $crate::BoundedInteger>::Repr) -> $ty { *self - *rhs }
        }
    }
}

/// Implements `std::ops::Mul` for a `BoundedInteger` enum with `Self`.
///
/// Implements the following combinations. The `Output` is always `Self`.
///
/// - `Self * Self`
/// - `Self * &Self`
/// - `&Self * Self`
/// - `&Self * &Self`
///
/// The implementations always panic on overflow.
#[macro_export]
macro_rules! bounded_integer_mul_self_impls {
    ($ty:ty) => {
        impl ::std::ops::Mul<$ty> for $ty {
            type Output = Self;
            fn mul(self, rhs: Self) -> Self {
                use $crate::BoundedInteger;
                self.checked_mul(rhs).expect("arithmetic operation overflowed")
            }
        }

        impl<'a> ::std::ops::Mul<&'a $ty> for $ty {
            type Output = Self;
            fn mul(self, rhs: &Self) -> Self { self * *rhs }
        }

        impl<'a> ::std::ops::Mul<$ty> for &'a $ty {
            type Output = $ty;
            fn mul(self, rhs: $ty) -> $ty { *self * rhs }
        }

        impl<'a, 'b> ::std::ops::Mul<&'b $ty> for &'a $ty {
            type Output = $ty;
            fn mul(self, rhs: &$ty) -> $ty { *self * *rhs }
        }
    }
}

/// Implements `std::ops::Mul` for a `BoundedInteger` enum with `Self::Repr`.
///
/// Implements the following combinations. The `Output` is always `Self`.
///
/// - `Self * Self::Repr`
/// - `Self * &Self::Repr`
/// - `&Self * Self::Repr`
/// - `&Self * &Self::Repr`
///
/// The implementations always panic on overflow.
#[macro_export]
macro_rules! bounded_integer_mul_repr_impls {
    ($ty:ty) => {
        impl ::std::ops::Mul<<$ty as $crate::BoundedInteger>::Repr> for $ty {
            type Output = Self;
            fn mul(self, rhs: <$ty as $crate::BoundedInteger>::Repr) -> Self {
                use $crate::BoundedInteger;
                self.checked_mul_repr(rhs).expect("arithmetic operation overflowed")
            }
        }

        impl<'a> ::std::ops::Mul<&'a <$ty as $crate::BoundedInteger>::Repr> for $ty {
            type Output = Self;
            fn mul(self, rhs: &'a <$ty as $crate::BoundedInteger>::Repr) -> Self { self * *rhs }
        }

        impl<'a> ::std::ops::Mul<<$ty as $crate::BoundedInteger>::Repr> for &'a $ty {
            type Output = $ty;
            fn mul(self, rhs: <$ty as $crate::BoundedInteger>::Repr) -> $ty { *self * rhs }
        }

        impl<'a, 'b> ::std::ops::Mul<&'b <$ty as $crate::BoundedInteger>::Repr> for &'a $ty {
            type Output = $ty;
            fn mul(self, rhs: &<$ty as $crate::BoundedInteger>::Repr) -> $ty { *self * *rhs }
        }
    }
}

/// Implements `std::ops::Div` for a `BoundedInteger` enum with `Self`.
///
/// Implements the following combinations. The `Output` is always `Self`.
///
/// - `Self / Self`
/// - `Self / &Self`
/// - `&Self / Self`
/// - `&Self / &Self`
///
/// The implementations always panic on overflow.
#[macro_export]
macro_rules! bounded_integer_div_self_impls {
    ($ty:ty) => {
        impl ::std::ops::Div<$ty> for $ty {
            type Output = Self;
            fn div(self, rhs: Self) -> Self {
                use $crate::BoundedInteger;
                self.checked_div(rhs).expect("arithmetic operation overflowed")
            }
        }

        impl<'a> ::std::ops::Div<&'a $ty> for $ty {
            type Output = Self;
            fn div(self, rhs: &Self) -> Self { self / *rhs }
        }

        impl<'a> ::std::ops::Div<$ty> for &'a $ty {
            type Output = $ty;
            fn div(self, rhs: $ty) -> $ty { *self / rhs }
        }

        impl<'a, 'b> ::std::ops::Div<&'b $ty> for &'a $ty {
            type Output = $ty;
            fn div(self, rhs: &$ty) -> $ty { *self / *rhs }
        }
    }
}

/// Implements `std::ops::Div` for a `BoundedInteger` enum with `Self::Repr`.
///
/// Implements the following combinations. The `Output` is always `Self`.
///
/// - `Self / Self::Repr`
/// - `Self / &Self::Repr`
/// - `&Self / Self::Repr`
/// - `&Self / &Self::Repr`
///
/// The implementations always panic on overflow.
#[macro_export]
macro_rules! bounded_integer_div_repr_impls {
    ($ty:ty) => {
        impl ::std::ops::Div<<$ty as $crate::BoundedInteger>::Repr> for $ty {
            type Output = Self;
            fn div(self, rhs: <$ty as $crate::BoundedInteger>::Repr) -> Self {
                use $crate::BoundedInteger;
                self.checked_div_repr(rhs).expect("arithmetic operation overflowed")
            }
        }

        impl<'a> ::std::ops::Div<&'a <$ty as $crate::BoundedInteger>::Repr> for $ty {
            type Output = Self;
            fn div(self, rhs: &'a <$ty as $crate::BoundedInteger>::Repr) -> Self { self / *rhs }
        }

        impl<'a> ::std::ops::Div<<$ty as $crate::BoundedInteger>::Repr> for &'a $ty {
            type Output = $ty;
            fn div(self, rhs: <$ty as $crate::BoundedInteger>::Repr) -> $ty { *self / rhs }
        }

        impl<'a, 'b> ::std::ops::Div<&'b <$ty as $crate::BoundedInteger>::Repr> for &'a $ty {
            type Output = $ty;
            fn div(self, rhs: &<$ty as $crate::BoundedInteger>::Repr) -> $ty { *self / *rhs }
        }
    }
}
