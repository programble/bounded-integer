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

/// Marker trait for integer representations.
pub trait Repr: Copy { }

impl Repr for u8 { }
impl Repr for u16 { }
impl Repr for u32 { }
impl Repr for u64 { }

impl Repr for i8 { }
impl Repr for i16 { }
impl Repr for i32 { }
impl Repr for i64 { }

/// Bounded integer.
pub trait BoundedInteger: Copy + Eq + Ord {
    /// Integer representation.
    type Repr: Repr;

    /// Converts from representation to Self.
    fn from_repr(repr: Self::Repr) -> Option<Self>;

    /// Converts from Self to representation.
    fn to_repr(self) -> Self::Repr;
}

pub mod bit;
