#![allow(dead_code)]
#![feature(plugin)]
#![plugin(bounded_integer_plugin)]

use std::fmt::Debug;

trait AssertDefined { }
trait AssertImplDebug: Debug { }
trait AssertImplCopy: Copy { }
trait AssertImplEq: Eq { }
trait AssertImplOrd: Ord { }
trait AssertSizeOf<T> { fn assert(self) -> T; }
trait AssertVariants { fn assert(self); }

macro_rules! assert_size_of {
    ($a:ty, $b:ty) => {
        impl AssertSizeOf<$a> for $b {
            fn assert(self) -> $a {
                unsafe { std::mem::transmute(self) }
            }
        }
    }
}

macro_rules! assert_variants {
    ($a:ty { $($v:pat),+ }) => {
        impl AssertVariants for $a {
            fn assert(self) {
                match self {
                    $($v => ()),+
                }
            }
        }
    }
}

bounded_integer! { enum A: u8 { 0...5 } }
impl AssertDefined for A { }

mod b {
    bounded_integer! { pub enum B: u8 { 0...5 } }
}
impl AssertDefined for b::B { }

bounded_integer! {
    #[derive(Debug)]
    enum C: u8 { 0...5 }
}
impl AssertDefined for C { }
impl AssertImplDebug for C { }

bounded_integer! { enum D: u8 { 0...5 } }
impl AssertDefined for D { }
impl AssertImplCopy for D { }
impl AssertImplEq for D { }
impl AssertImplOrd for D { }

bounded_integer! { enum E: u8 { 0...5 } }
bounded_integer! { enum F: i8 { 0...5 } }
bounded_integer! { enum G: u16 { 0...5 } }
bounded_integer! { enum H: i16 { 0...5 } }
bounded_integer! { enum I: u32 { 0...5 } }
bounded_integer! { enum J: i32 { 0...5 } }
bounded_integer! { enum K: u64 { 0...5 } }
bounded_integer! { enum L: i64 { 0...5 } }

assert_size_of!(u8, E);
assert_size_of!(i8, F);
assert_size_of!(u16, G);
assert_size_of!(i16, H);
assert_size_of!(u32, I);
assert_size_of!(i32, J);
assert_size_of!(u64, K);
assert_size_of!(i64, L);

bounded_integer! { enum M: i8 { 0...0 } }
bounded_integer! { enum N: i8 { -3...3 } }

assert_variants!(M { M::Z0 });
assert_variants!(N { N::N3, N::N2, N::N1, N::Z0, N::P1, N::P2, N::P3 });

#[test]
fn m_variants() {
    assert_eq!(0, M::Z0 as i8);
}

#[test]
fn n_variants() {
    assert_eq!(-3, N::N3 as i8);
    assert_eq!(-2, N::N2 as i8);
    assert_eq!(-1, N::N1 as i8);
    assert_eq!(0, N::Z0 as i8);
    assert_eq!(1, N::P1 as i8);
    assert_eq!(2, N::P2 as i8);
    assert_eq!(3, N::P3 as i8);
}
