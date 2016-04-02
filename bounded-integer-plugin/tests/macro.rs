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

macro_rules! assert_size_of {
    ($a:ty, $b:ty) => {
        impl AssertSizeOf<$a> for $b {
            fn assert(self) -> $a {
                unsafe { std::mem::transmute(self) }
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
