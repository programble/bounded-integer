#[macro_use]
extern crate bounded_integer;

use bounded_integer::BoundedInteger;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
enum Bit { Z0, P1 }
bounded_integer_impls!(Bit, u8, Bit::Z0, Bit::P1);

// Tests overflow of the representation.
#[test]
fn checked_sub() {
    assert_eq!(None, Bit::Z0.checked_sub(Bit::P1));
}
