#![feature(plugin)]
#![plugin(bounded_integer_plugin)]

#[macro_use]
extern crate bounded_integer;

use std::fmt::Debug;
use std::ops::{Add, Sub, Mul, Div, Rem, Neg};

use bounded_integer::{BoundedInteger, Repr};

bounded_integer! { enum Bit: u8 { 0...1 } }
bounded_integer! { enum UNibble: u8 { 0...15 } }
bounded_integer! { enum SNibble: i8 { -8...7 } }
bounded_integer! { enum NZUNibble: u8 { 1...15 } }

fn assert_impl_debug<T>() where T: Debug { }
fn assert_impl_copy<T>() where T: Copy { }
fn assert_impl_eq<T>() where T: Eq { }
fn assert_impl_ord<T>() where T: Ord { }
fn assert_impl_bounded_integer<T>() where T: BoundedInteger { }
fn assert_impl_into_repr<T, U>() where T: BoundedInteger, U: Into<T::Repr> { }
fn assert_impl_add_self<T>() where T: Add<T> { }
fn assert_impl_add_repr<T, U>() where T: BoundedInteger, U: Add<T::Repr> { }
fn assert_impl_sub_self<T>() where T: Sub<T> { }
fn assert_impl_sub_repr<T, U>() where T: BoundedInteger, U: Sub<T::Repr> { }
fn assert_impl_mul_self<T>() where T: Mul<T> { }
fn assert_impl_mul_repr<T, U>() where T: BoundedInteger, U: Mul<T::Repr> { }
fn assert_impl_div_self<T>() where T: Div<T> { }
fn assert_impl_div_repr<T, U>() where T: BoundedInteger, U: Div<T::Repr> { }
fn assert_impl_rem_self<T>() where T: Rem<T> { }
fn assert_impl_rem_repr<T, U>() where T: BoundedInteger, U: Rem<T::Repr> { }
fn assert_impl_neg<T>() where T: Neg { }

fn assert_bounded_integer_repr<I, R>() where I: BoundedInteger<Repr = R>, R: Repr { }

#[test]
fn bit() {
    assert_impl_debug::<Bit>();
    assert_impl_copy::<Bit>();
    assert_impl_eq::<Bit>();
    assert_impl_bounded_integer::<Bit>();
    assert_impl_into_repr::<Bit, Bit>();
    assert_impl_add_self::<Bit>();
    assert_impl_add_repr::<Bit, Bit>();
    assert_impl_sub_self::<Bit>();
    assert_impl_sub_repr::<Bit, Bit>();
    assert_impl_mul_self::<Bit>();
    assert_impl_mul_repr::<Bit, Bit>();
    assert_impl_div_self::<Bit>();
    assert_impl_div_repr::<Bit, Bit>();
    assert_impl_rem_self::<Bit>();
    assert_impl_rem_repr::<Bit, Bit>();
    assert_impl_neg::<Bit>();

    assert_bounded_integer_repr::<Bit, u8>();
}

#[test]
fn unibble() {
    assert_impl_debug::<UNibble>();
    assert_impl_copy::<UNibble>();
    assert_impl_eq::<UNibble>();
    assert_impl_bounded_integer::<UNibble>();
    assert_impl_into_repr::<UNibble, UNibble>();
    assert_impl_add_self::<UNibble>();
    assert_impl_add_repr::<UNibble, UNibble>();
    assert_impl_sub_self::<UNibble>();
    assert_impl_sub_repr::<UNibble, UNibble>();
    assert_impl_mul_self::<UNibble>();
    assert_impl_mul_repr::<UNibble, UNibble>();
    assert_impl_div_self::<UNibble>();
    assert_impl_div_repr::<UNibble, UNibble>();
    assert_impl_rem_self::<UNibble>();
    assert_impl_rem_repr::<UNibble, UNibble>();
    assert_impl_neg::<UNibble>();

    assert_bounded_integer_repr::<UNibble, u8>();
}

#[test]
fn snibble() {
    assert_impl_debug::<SNibble>();
    assert_impl_copy::<SNibble>();
    assert_impl_eq::<SNibble>();
    assert_impl_bounded_integer::<SNibble>();
    assert_impl_into_repr::<SNibble, SNibble>();
    assert_impl_add_self::<SNibble>();
    assert_impl_add_repr::<SNibble, SNibble>();
    assert_impl_sub_self::<SNibble>();
    assert_impl_sub_repr::<SNibble, SNibble>();
    assert_impl_mul_self::<SNibble>();
    assert_impl_mul_repr::<SNibble, SNibble>();
    assert_impl_div_self::<SNibble>();
    assert_impl_div_repr::<SNibble, SNibble>();
    assert_impl_rem_self::<SNibble>();
    assert_impl_rem_repr::<SNibble, SNibble>();
    assert_impl_neg::<SNibble>();

    assert_bounded_integer_repr::<SNibble, i8>();
}

#[test]
fn nzunibble() {
    assert_impl_debug::<NZUibble>();
    assert_impl_copy::<NZUibble>();
    assert_impl_eq::<NZUibble>();
    assert_impl_bounded_integer::<NZUibble>();
    assert_impl_into_repr::<NZUibble, NZUibble>();
    assert_impl_add_self::<NZUibble>();
    assert_impl_add_repr::<NZUibble, NZUibble>();
    assert_impl_sub_self::<NZUibble>();
    assert_impl_sub_repr::<NZUibble, NZUibble>();
    assert_impl_mul_self::<NZUibble>();
    assert_impl_mul_repr::<NZUibble, NZUibble>();
    assert_impl_div_self::<NZUibble>();
    assert_impl_div_repr::<NZUibble, NZUibble>();
    assert_impl_rem_self::<NZUibble>();
    assert_impl_rem_repr::<NZUibble, NZUibble>();
    assert_impl_neg::<NZUibble>();

    assert_bounded_integer_repr::<NZUibble, u8>();
}
