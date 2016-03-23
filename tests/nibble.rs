#[macro_use]
extern crate bounded_integer;

use bounded_integer::BoundedInteger;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord)]
#[allow(dead_code)]
#[repr(u8)]
enum Nibble {
    N8 = 248, N7, N6, N5, N4, N3, N2, N1, U0 = 0, P1, P2, P3, P4, P5, P6, P7
}
bounded_integer_impls!(Nibble, i8, Nibble::N8, Nibble::P7);
bounded_integer_partial_ord_impl!(Nibble);
