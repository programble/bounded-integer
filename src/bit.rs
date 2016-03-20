//! Bits.

/// A bit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(missing_docs)]
#[repr(u8)]
pub enum Bit { U0, U1 }
bounded_integer_impl!(Bit, u8, Bit::U0, Bit::U1);

#[cfg(test)]
mod tests {
    use BoundedInteger;
    use super::Bit;

    #[test]
    fn from_repr() {
        assert_eq!(Some(Bit::U0), Bit::from_repr(0));
        assert_eq!(Some(Bit::U1), Bit::from_repr(1));
        assert_eq!(None, Bit::from_repr(2));
    }

    #[test]
    fn to_repr() {
        assert_eq!(0, Bit::U0.to_repr());
        assert_eq!(1, Bit::U1.to_repr());
    }

    #[test]
    fn min_value() {
        assert_eq!(Bit::U0, Bit::min_value());
    }

    #[test]
    fn max_value() {
        assert_eq!(Bit::U1, Bit::max_value());
    }

    #[test]
    fn checked_add() {
        assert_eq!(Some(Bit::U0), Bit::U0.checked_add(Bit::U0));
        assert_eq!(Some(Bit::U1), Bit::U0.checked_add(Bit::U1));
        assert_eq!(Some(Bit::U1), Bit::U1.checked_add(Bit::U0));
        assert_eq!(None, Bit::U1.checked_add(Bit::U1));
    }

    #[test]
    fn checked_add_repr() {
        assert_eq!(Some(Bit::U0), Bit::U0.checked_add_repr(0));
        assert_eq!(Some(Bit::U1), Bit::U0.checked_add_repr(1));
        assert_eq!(Some(Bit::U1), Bit::U1.checked_add_repr(0));
        assert_eq!(None, Bit::U1.checked_add_repr(1));
        assert_eq!(None, Bit::U0.checked_add_repr(2));
    }

    #[test]
    fn checked_sub() {
        assert_eq!(Some(Bit::U0), Bit::U0.checked_sub(Bit::U0));
        assert_eq!(Some(Bit::U1), Bit::U1.checked_sub(Bit::U0));
        assert_eq!(Some(Bit::U0), Bit::U1.checked_sub(Bit::U1));
        assert_eq!(None, Bit::U0.checked_sub(Bit::U1));
    }

    #[test]
    fn checked_sub_repr() {
        assert_eq!(Some(Bit::U0), Bit::U0.checked_sub_repr(0));
        assert_eq!(Some(Bit::U1), Bit::U1.checked_sub_repr(0));
        assert_eq!(Some(Bit::U0), Bit::U1.checked_sub_repr(1));
        assert_eq!(None, Bit::U0.checked_sub_repr(1));
        assert_eq!(None, Bit::U0.checked_sub_repr(2));
        assert_eq!(None, Bit::U1.checked_sub_repr(2));
    }

    #[test]
    fn saturating_add() {
        assert_eq!(Bit::U1, Bit::U1.saturating_add(Bit::U1));
    }

    #[test]
    fn saturating_add_repr() {
        assert_eq!(Bit::U1, Bit::U1.saturating_add_repr(1));
        assert_eq!(Bit::U1, Bit::U0.saturating_add_repr(2));
    }

    #[test]
    fn saturating_sub() {
        assert_eq!(Bit::U0, Bit::U0.saturating_sub(Bit::U1));
    }

    #[test]
    fn saturating_sub_repr() {
        assert_eq!(Bit::U0, Bit::U0.saturating_sub_repr(1));
        assert_eq!(Bit::U0, Bit::U1.saturating_sub_repr(2));
    }
}
