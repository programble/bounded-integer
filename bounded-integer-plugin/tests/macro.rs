#![feature(plugin)]
#![plugin(bounded_integer_plugin)]

#[macro_use]
extern crate bounded_integer;

use std::fmt::Debug;
use std::ops::{Add, Sub, Mul, Div, Rem, Neg};

use bounded_integer::{BoundedInteger, Repr};

bounded_integer! {
    pub enum Bit { 0...1 }
}

bounded_integer! {
    enum UNibble { 0...15 }
}

bounded_integer! {
    enum SNibble { -8...7 }
}

bounded_integer! {
    enum NZUNibble { 1...15 }
}

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
