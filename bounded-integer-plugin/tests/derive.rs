#![allow(dead_code)]
#![feature(plugin)]
#![plugin(bounded_integer_plugin)]

trait AssertImplCopy: Copy { }
trait AssertImplEq: Eq { }
trait AssertImplOrd: Ord { }

bounded_integer! { enum A: u8 { 0...1 } }
impl AssertImplCopy for A { }
impl AssertImplEq for A { }
impl AssertImplOrd for A { }
