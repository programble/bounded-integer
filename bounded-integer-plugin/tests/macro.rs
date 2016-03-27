#![allow(dead_code)]
#![feature(plugin)]
#![plugin(bounded_integer_plugin)]

use std::fmt::Debug;

trait AssertDefined { }
trait AssertImplDebug: Debug { }
trait AssertImplCopy: Copy { }
trait AssertImplEq: Eq { }
trait AssertImplOrd: Ord { }

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
