#![feature(plugin)]
#![plugin(bounded_integer_plugin)]

#[macro_use]
extern crate bounded_integer;

use std::ops::{Add, Sub, Mul, Div, Rem, Neg};

use bounded_integer::{BoundedInteger, Repr};

trait AssertImplBoundedInteger: BoundedInteger { }
trait AssertBoundedIntegerRepr<R: Repr>: BoundedInteger<Repr = R> { }
trait AssertImplInto<T>: Into<T> { }
trait AssertImplAdd<RHS>: Add<RHS> { }
trait AssertImplSub<RHS>: Sub<RHS> { }
trait AssertImplMul<RHS>: Mul<RHS> { }
trait AssertImplDiv<RHS>: Div<RHS> { }
trait AssertImplRem<RHS>: Rem<RHS> { }
trait AssertImplNeg: Neg { }

bounded_integer! {
    #[derive(Debug)]
    enum A: i8 { -5...5 }
}

impl AssertImplBoundedInteger for A { }
impl AssertBoundedIntegerRepr<i8> for A { }

impl AssertImplInto<i8> for A { }

impl AssertImplAdd<A> for A { }
impl<'a> AssertImplAdd<&'a A> for A { }
impl<'a> AssertImplAdd<A> for &'a A { }
impl<'a, 'b> AssertImplAdd<&'a A> for &'b A { }

impl AssertImplAdd<i8> for A { }
impl<'a> AssertImplAdd<&'a i8> for A { }
impl<'a> AssertImplAdd<i8> for &'a A { }
impl<'a, 'b> AssertImplAdd<&'a i8> for &'b A { }

impl AssertImplSub<A> for A { }
impl<'a> AssertImplSub<&'a A> for A { }
impl<'a> AssertImplSub<A> for &'a A { }
impl<'a, 'b> AssertImplSub<&'a A> for &'b A { }

impl AssertImplSub<i8> for A { }
impl<'a> AssertImplSub<&'a i8> for A { }
impl<'a> AssertImplSub<i8> for &'a A { }
impl<'a, 'b> AssertImplSub<&'a i8> for &'b A { }

impl AssertImplMul<A> for A { }
impl<'a> AssertImplMul<&'a A> for A { }
impl<'a> AssertImplMul<A> for &'a A { }
impl<'a, 'b> AssertImplMul<&'a A> for &'b A { }

impl AssertImplMul<i8> for A { }
impl<'a> AssertImplMul<&'a i8> for A { }
impl<'a> AssertImplMul<i8> for &'a A { }
impl<'a, 'b> AssertImplMul<&'a i8> for &'b A { }

impl AssertImplDiv<A> for A { }
impl<'a> AssertImplDiv<&'a A> for A { }
impl<'a> AssertImplDiv<A> for &'a A { }
impl<'a, 'b> AssertImplDiv<&'a A> for &'b A { }

impl AssertImplDiv<i8> for A { }
impl<'a> AssertImplDiv<&'a i8> for A { }
impl<'a> AssertImplDiv<i8> for &'a A { }
impl<'a, 'b> AssertImplDiv<&'a i8> for &'b A { }

impl AssertImplRem<A> for A { }
impl<'a> AssertImplRem<&'a A> for A { }
impl<'a> AssertImplRem<A> for &'a A { }
impl<'a, 'b> AssertImplRem<&'a A> for &'b A { }

impl AssertImplRem<i8> for A { }
impl<'a> AssertImplRem<&'a i8> for A { }
impl<'a> AssertImplRem<i8> for &'a A { }
impl<'a, 'b> AssertImplRem<&'a i8> for &'b A { }

impl AssertImplNeg for A { }
impl<'a> AssertImplNeg for &'a A { }

#[test]
fn from_repr() {
    assert_eq!(Some(A::N3), A::from_repr(-3));
}

#[test]
fn to_repr() {
    assert_eq!(-3, A::N3.to_repr());
}

#[test]
fn min_value() {
    assert_eq!(A::N5, A::min_value());
}

#[test]
fn max_value() {
    assert_eq!(A::P5, A::max_value());
}
