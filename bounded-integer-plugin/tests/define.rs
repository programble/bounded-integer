#![allow(dead_code)]
#![feature(plugin)]
#![plugin(bounded_integer_plugin)]

#[macro_use]
extern crate bounded_integer;

trait AssertDefined { }

bounded_integer! { enum A: u8 { 0...1 } }
impl AssertDefined for A { }

mod b {
    bounded_integer! { pub enum B: u8 { 0...1 } }
}
impl AssertDefined for b::B { }
