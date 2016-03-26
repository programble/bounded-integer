#![feature(plugin)]
#![plugin(bounded_integer_plugin)]

#[macro_use]
extern crate bounded_integer;

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
