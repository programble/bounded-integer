//! Integer representations.

use std::ops::Add;

/// Integer representation.
///
/// Should not be implemented for any new types.
pub trait Repr: Copy + Eq + Ord + Add<Self, Output=Self> {
    /// Returns true if negative.
    fn is_negative(self) -> bool { false }
}

impl Repr for u8 { }
impl Repr for u16 { }
impl Repr for u32 { }
impl Repr for u64 { }

macro_rules! impl_signed {
    ($ty:ty) => {
        impl Repr for $ty {
            fn is_negative(self) -> bool {
                self.is_negative()
            }
        }
    }
}

impl_signed!(i8);
impl_signed!(i16);
impl_signed!(i32);
impl_signed!(i64);
