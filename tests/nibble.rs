#[macro_use]
extern crate bounded_integer;

use bounded_integer::BoundedInteger;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord)]
#[allow(dead_code)]
#[repr(u8)]
enum Nibble {
    N8 = 248, N7, N6, N5, N4, N3, N2, N1, U0 = 0, P1, P2, P3, P4, P5, P6, P7
}

bounded_integer_impl!(Nibble, i8, Nibble::N8, Nibble::P7);
bounded_integer_partial_ord_impl!(Nibble);
bounded_integer_mul_self_impls!(Nibble);
bounded_integer_mul_repr_impls!(Nibble);

#[test]
fn checked_mul() {
    assert_eq!(Some(Nibble::P4), Nibble::P2.checked_mul(Nibble::P2));
    assert_eq!(Some(Nibble::N4), Nibble::P2.checked_mul(Nibble::N2));
    assert_eq!(None, Nibble::P4.checked_mul(Nibble::P2));
    assert_eq!(None, Nibble::N8.checked_mul(Nibble::P2));
}

#[test]
fn checked_mul_repr() {
    assert_eq!(Some(Nibble::P4), Nibble::P2.checked_mul_repr(2));
    assert_eq!(Some(Nibble::N4), Nibble::P2.checked_mul_repr(-2));
    assert_eq!(None, Nibble::P4.checked_mul_repr(2));
    assert_eq!(None, Nibble::N8.checked_mul_repr(2));
}

#[test]
fn saturating_mul() {
    assert_eq!(Nibble::P7, Nibble::P4.saturating_mul(Nibble::P2));
    assert_eq!(Nibble::P7, Nibble::N4.saturating_mul(Nibble::N2));
    assert_eq!(Nibble::N8, Nibble::N8.saturating_mul(Nibble::P2));
    assert_eq!(Nibble::N8, Nibble::P2.saturating_mul(Nibble::N8));
}

#[test]
fn saturating_mul_repr() {
    assert_eq!(Nibble::P7, Nibble::P4.saturating_mul_repr(2));
    assert_eq!(Nibble::P7, Nibble::N4.saturating_mul_repr(-2));
    assert_eq!(Nibble::N8, Nibble::N8.saturating_mul_repr(2));
    assert_eq!(Nibble::N8, Nibble::P2.saturating_mul_repr(-8));
}

#[test]
fn ops_mul_self() {
    assert_eq!(Nibble::P4, Nibble::P2 * Nibble::P2);
    assert_eq!(Nibble::P4, Nibble::P2 * &Nibble::P2);
    assert_eq!(Nibble::P4, &Nibble::P2 * Nibble::P2);
    assert_eq!(Nibble::P4, &Nibble::P2 * &Nibble::P2);
}

#[test]
#[should_panic]
fn ops_mul_self_overflow() {
    let _ = Nibble::P4 * Nibble::P4;
}

#[test]
fn ops_mul_repr() {
    assert_eq!(Nibble::P4, Nibble::P2 * 2);
    assert_eq!(Nibble::P4, Nibble::P2 * &2);
    assert_eq!(Nibble::P4, &Nibble::P2 * 2);
    assert_eq!(Nibble::P4, &Nibble::P2 * &2);
}

#[test]
#[should_panic]
fn ops_mul_repr_overflow() {
    let _ = Nibble::P4 * 4;
}
