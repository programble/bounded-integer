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

    /// Checked integer remainder.
    fn checked_rem(self, other: Self) -> Option<Self>;

    /// Checked integer negation.
    fn checked_neg(self) -> Option<Self>;
}

macro_rules! repr_impl {
    ($i:ident @ $ty:ty) => {
        impl Repr for $ty {
            repr_impl!($i);
            fn checked_add(self, other: Self) -> Option<Self> { self.checked_add(other) }
            fn checked_sub(self, other: Self) -> Option<Self> { self.checked_sub(other) }
            fn checked_mul(self, other: Self) -> Option<Self> { self.checked_mul(other) }
            fn checked_div(self, other: Self) -> Option<Self> { self.checked_div(other) }
            fn checked_rem(self, other: Self) -> Option<Self> { self.checked_rem(other) }
            fn checked_neg(self) -> Option<Self> { self.checked_neg() }
        }
    };

    (u) => { fn is_negative(self) -> bool { false } };
    (i) => { fn is_negative(self) -> bool { self.is_negative() } };
}

repr_impl!(u @ u8);
repr_impl!(u @ u16);
repr_impl!(u @ u32);
repr_impl!(u @ u64);

repr_impl!(i @ i8);
repr_impl!(i @ i16);
repr_impl!(i @ i32);
repr_impl!(i @ i64);
