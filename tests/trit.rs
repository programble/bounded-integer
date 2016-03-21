#[macro_use]
extern crate bounded_integer;

use bounded_integer::BoundedInteger;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord)]
#[repr(u8)]
enum Trit {
    N1 = 255,
    U0 = 0,
    P1 = 1,
}

bounded_integer_impl!(Trit, i8, Trit::N1, Trit::P1);
bounded_integer_partial_ord_impl!(Trit);
bounded_integer_into_repr_impl!(Trit);
bounded_integer_add_self_impls!(Trit);
bounded_integer_add_repr_impls!(Trit);
bounded_integer_sub_self_impls!(Trit);
bounded_integer_sub_repr_impls!(Trit);

#[test]
fn from_repr() {
    assert_eq!(None, Trit::from_repr(-2));
    assert_eq!(Some(Trit::N1), Trit::from_repr(-1));
    assert_eq!(Some(Trit::U0), Trit::from_repr(0));
    assert_eq!(Some(Trit::P1), Trit::from_repr(1));
    assert_eq!(None, Trit::from_repr(2));
}

#[test]
fn to_repr() {
    assert_eq!(-1, Trit::N1.to_repr());
    assert_eq!(0, Trit::U0.to_repr());
    assert_eq!(1, Trit::P1.to_repr());
}

#[test]
fn min_value() {
    assert_eq!(Trit::N1, Trit::min_value());
}

#[test]
fn max_value() {
    assert_eq!(Trit::P1, Trit::max_value());
}

#[test]
fn checked_add() {
    assert_eq!(Some(Trit::U0), Trit::N1.checked_add(Trit::P1));
    assert_eq!(Some(Trit::U0), Trit::P1.checked_add(Trit::N1));
    assert_eq!(None, Trit::P1.checked_add(Trit::P1));
    assert_eq!(None, Trit::N1.checked_add(Trit::N1));
}

#[test]
fn checked_add_repr() {
    assert_eq!(None, Trit::U0.checked_add_repr(2));
    assert_eq!(None, Trit::U0.checked_add_repr(-2));
}

#[test]
fn checked_sub() {
    assert_eq!(Some(Trit::U0), Trit::P1.checked_sub(Trit::P1));
    assert_eq!(Some(Trit::U0), Trit::N1.checked_sub(Trit::N1));
    assert_eq!(None, Trit::N1.checked_sub(Trit::P1));
    assert_eq!(None, Trit::P1.checked_sub(Trit::N1));
}

#[test]
fn checked_sub_repr() {
    assert_eq!(None, Trit::U0.checked_sub_repr(2));
    assert_eq!(None, Trit::U0.checked_sub_repr(-2));
}

#[test]
fn saturating_add() {
    assert_eq!(Trit::P1, Trit::P1.saturating_add(Trit::P1));
    assert_eq!(Trit::N1, Trit::N1.saturating_add(Trit::N1));
}

#[test]
fn saturating_add_repr() {
    assert_eq!(Trit::P1, Trit::U0.saturating_add_repr(2));
    assert_eq!(Trit::N1, Trit::U0.saturating_add_repr(-2));
}

#[test]
fn saturating_sub() {
    assert_eq!(Trit::N1, Trit::N1.saturating_sub(Trit::P1));
    assert_eq!(Trit::P1, Trit::P1.saturating_sub(Trit::N1));
}

#[test]
fn saturating_sub_repr() {
    assert_eq!(Trit::N1, Trit::U0.saturating_sub_repr(2));
    assert_eq!(Trit::P1, Trit::U0.saturating_sub_repr(-2));
}

#[test]
fn into() {
    assert_eq!(1i8, Trit::P1.into());
}

#[test]
fn ops_add_self() {
    assert_eq!(Trit::P1, Trit::U0 + Trit::P1);
    assert_eq!(Trit::P1, Trit::U0 + &Trit::P1);
    assert_eq!(Trit::P1, &Trit::U0 + Trit::P1);
    assert_eq!(Trit::P1, &Trit::U0 + &Trit::P1);
}

#[test]
#[should_panic]
fn ops_add_self_overflow() {
    let _ = Trit::P1 + Trit::P1;
}

#[test]
fn ops_add_repr() {
    assert_eq!(Trit::P1, Trit::U0 + 1);
    assert_eq!(Trit::P1, Trit::U0 + &1);
    assert_eq!(Trit::P1, &Trit::U0 + 1);
    assert_eq!(Trit::P1, &Trit::U0 + &1);
}

#[test]
#[should_panic]
fn ops_add_repr_overflow() {
    let _ = Trit::P1 + 1;
}

#[test]
fn ops_sub_self() {
    assert_eq!(Trit::U0, Trit::P1 - Trit::P1);
    assert_eq!(Trit::U0, Trit::P1 - &Trit::P1);
    assert_eq!(Trit::U0, &Trit::P1 - Trit::P1);
    assert_eq!(Trit::U0, &Trit::P1 - &Trit::P1);
}

#[test]
#[should_panic]
fn ops_sub_self_overflow() {
    let _ = Trit::N1 - Trit::P1;
}

#[test]
fn ops_sub_repr() {
    assert_eq!(Trit::U0, Trit::P1 - 1);
    assert_eq!(Trit::U0, Trit::P1 - &1);
    assert_eq!(Trit::U0, &Trit::P1 - 1);
    assert_eq!(Trit::U0, &Trit::P1 - &1);
}

#[test]
#[should_panic]
fn ops_sub_repr_overflow() {
    let _ = Trit::N1 - 1;
}
