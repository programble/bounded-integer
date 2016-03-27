#![allow(dead_code)]
#![feature(plugin)]
#![plugin(bounded_integer_plugin)]

use std::hash::Hash;

trait AssertDefined { }
trait AssertImplHash: Hash { }

bounded_integer! { enum A: u8 { 0...5 } }
impl AssertDefined for A { }

mod b {
    bounded_integer! { pub enum B: u8 { 0...5 } }
}
impl AssertDefined for b::B { }

bounded_integer! {
    #[derive(Hash)]
    enum C: u8 { 0...5 }
}
impl AssertDefined for C { }
impl AssertImplHash for C { }
