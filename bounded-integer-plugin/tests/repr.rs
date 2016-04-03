#![allow(dead_code)]
#![feature(plugin)]
#![plugin(bounded_integer_plugin)]

trait AssertSizeOf<T> {
    fn assert(self) -> T;
}

macro_rules! assert_size_of {
    ($a:ty, $b:ty) => {
        impl AssertSizeOf<$a> for $b {
            fn assert(self) -> $a {
                unsafe { std::mem::transmute(self) }
            }
        }
    }
}

bounded_integer! { enum A: u8 { 0...1 } }
bounded_integer! { enum B: i8 { 0...1 } }
bounded_integer! { enum C: u16 { 0...1 } }
bounded_integer! { enum D: i16 { 0...1 } }
bounded_integer! { enum E: u32 { 0...1 } }
bounded_integer! { enum F: i32 { 0...1 } }
bounded_integer! { enum G: u64 { 0...1 } }
bounded_integer! { enum H: i64 { 0...1 } }

assert_size_of!(u8, A);
assert_size_of!(i8, B);
assert_size_of!(u16, C);
assert_size_of!(i16, D);
assert_size_of!(u32, E);
assert_size_of!(i32, F);
assert_size_of!(u64, G);
assert_size_of!(i64, H);
