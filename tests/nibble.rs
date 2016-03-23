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

#[test]
fn from_repr() {
    assert_eq!(Some(Nibble::U0), Nibble::from_repr(0i8));
    assert_eq!(Some(Nibble::N8), Nibble::from_repr(-8i8));
    assert_eq!(Some(Nibble::P7), Nibble::from_repr(7i8));
}

#[test]
fn to_repr() {
    assert_eq!(0i8, Nibble::U0.to_repr());
    assert_eq!(-8i8, Nibble::N8.to_repr());
    assert_eq!(7i8, Nibble::P7.to_repr());
}

#[test]
fn min_value() {
    assert_eq!(Nibble::N8, Nibble::min_value());
}

#[test]
fn max_value() {
    assert_eq!(Nibble::P7, Nibble::max_value());
}

#[test]
fn checked_add() {
    assert_eq!(Some(Nibble::P3), Nibble::P1.checked_add(Nibble::P2));
    assert_eq!(Some(Nibble::N3), Nibble::N1.checked_add(Nibble::N2));
    assert_eq!(Some(Nibble::N1), Nibble::P1.checked_add(Nibble::N2));
    assert_eq!(None, Nibble::P7.checked_add(Nibble::P1));
    assert_eq!(None, Nibble::N8.checked_add(Nibble::N1));
}

#[test]
fn checked_sub() {
    assert_eq!(Some(Nibble::P1), Nibble::P3.checked_sub(Nibble::P2));
    assert_eq!(Some(Nibble::N1), Nibble::N3.checked_sub(Nibble::N2));
    assert_eq!(Some(Nibble::P1), Nibble::N1.checked_sub(Nibble::N2));
    assert_eq!(None, Nibble::N8.checked_sub(Nibble::P1));
    assert_eq!(None, Nibble::P7.checked_sub(Nibble::N1));
}
