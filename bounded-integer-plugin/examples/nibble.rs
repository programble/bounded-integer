// This example should be kept in sync with the documentation example in bounded-integer.

#![feature(plugin)]
#![plugin(bounded_integer_plugin)]

#[macro_use]
extern crate bounded_integer;

bounded_integer! {
    /// Value that can fit in a nibble.
    #[derive(Debug)]
    pub enum Nibble: i8 { -8...7 }
}

fn main() { }
