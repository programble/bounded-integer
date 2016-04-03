#![allow(dead_code)]
#![feature(plugin)]
#![plugin(bounded_integer_plugin)]

trait AssertVariants {
    fn assert(self);
}

macro_rules! assert_variants {
    ($t:ty { $($v:pat),+ }) => {
        impl AssertVariants for $t {
            fn assert(self) {
                match self {
                    $($v => ()),+
                }
            }
        }
    }
}

bounded_integer! { enum A: i8 { 0...0 } }
bounded_integer! { enum B: i8 { -2...2 } }

assert_variants!(A { A::Z0 });
assert_variants!(B { B::N2, B::N1, B::Z0, B::P1, B::P2 });

#[test]
fn a_variants() {
    assert_eq!(0, A::Z0 as i8);
}

#[test]
fn b_variants() {
    assert_eq!(-2, B::N2 as i8);
    assert_eq!(-1, B::N1 as i8);
    assert_eq!(0, B::Z0 as i8);
    assert_eq!(1, B::P1 as i8);
    assert_eq!(2, B::P2 as i8);
}
