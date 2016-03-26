#![feature(plugin)]
#![plugin(bounded_integer_plugin)]

bounded_integer! { enum Bit: u8 { 0...1 } }

mod module {
    bounded_integer! { pub enum Public: u8 { 1...2 } }
}

trait AssertDefined<T> { }

impl AssertDefined<Bit> { }
impl AssertDefined<module::Public> { }
