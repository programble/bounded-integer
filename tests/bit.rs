#[macro_use]
extern crate bounded_integer;

use bounded_integer::BoundedInteger;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
enum Bit { U0, U1 }
bounded_integer_impl!(Bit, u8, Bit::U0, Bit::U1);

// Tests overflow of the representation.
#[test]
fn checked_sub() {
    assert_eq!(None, Bit::U0.checked_sub(Bit::U1));
}
