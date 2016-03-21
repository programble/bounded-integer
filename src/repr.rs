//! Integer representations.

/// Integer representation.
///
/// Should not be implemented for any new types.
pub trait Repr: Copy + Eq + Ord {
    /// Returns true if negative.
    ///
    /// Used to determine to which bound operations should saturate.
    fn is_negative(self) -> bool;

    /// Checked integer addition.
    fn checked_add(self, other: Self) -> Option<Self>;

    /// Checked integer subtraction.
    fn checked_sub(self, other: Self) -> Option<Self>;

    /// Checked integer multiplication.
    fn checked_mul(self, other: Self) -> Option<Self>;

    /// Checked integer division.
    fn checked_div(self, other: Self) -> Option<Self>;
}

macro_rules! impl_unsigned {
    ($ty:ty) => {
        impl Repr for $ty {
            fn is_negative(self) -> bool { false }
            fn checked_add(self, other: Self) -> Option<Self> { self.checked_add(other) }
            fn checked_sub(self, other: Self) -> Option<Self> { self.checked_sub(other) }
            fn checked_mul(self, other: Self) -> Option<Self> { self.checked_mul(other) }
            fn checked_div(self, other: Self) -> Option<Self> { self.checked_div(other) }
        }
    }
}

macro_rules! impl_signed {
    ($ty:ty) => {
        impl Repr for $ty {
            fn is_negative(self) -> bool { self.is_negative() }
            fn checked_add(self, other: Self) -> Option<Self> { self.checked_add(other) }
            fn checked_sub(self, other: Self) -> Option<Self> { self.checked_sub(other) }
            fn checked_mul(self, other: Self) -> Option<Self> { self.checked_mul(other) }
            fn checked_div(self, other: Self) -> Option<Self> { self.checked_div(other) }
        }
    }
}

impl_unsigned!(u8);
impl_unsigned!(u16);
impl_unsigned!(u32);
impl_unsigned!(u64);

impl_signed!(i8);
impl_signed!(i16);
impl_signed!(i32);
impl_signed!(i64);
