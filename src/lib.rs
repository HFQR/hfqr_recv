mod order;
mod tick;
mod unit;

pub mod parse;

use core::{mem::transmute, ops::Add};

pub use crate::order::OrderLike;
pub use crate::tick::TickLike;
pub use crate::unit::*;

/// identifier for market depth of ask/bid
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Depth {
    /// ask/bid 1
    _1 = 0,
    /// ask/bid 2
    _2,
    /// ask/bid 3
    _3,
    /// ask/bid 4
    _4,
    /// ask/bid 5
    _5,
}

impl Add<u8> for Depth {
    type Output = Self;

    fn add(self, rhs: u8) -> Self::Output {
        let res = self as u8 + rhs;
        assert!(res < 5, "Depth overflow");
        // SAFETY
        // just checked with assert
        unsafe { transmute(res) }
    }
}

impl Add for Depth {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self + (rhs as u8) + 1
    }
}

#[cfg(test)]
mod test {
    use std::panic::catch_unwind;

    use super::Depth;

    #[test]
    fn depth() {
        assert_eq!(0, Depth::_1 as u8);
        assert_eq!(1, Depth::_2 as u8);
        assert_eq!(2, Depth::_3 as u8);
        assert_eq!(3, Depth::_4 as u8);
        assert_eq!(4, Depth::_5 as u8);

        let dep = Depth::_1;

        assert_eq!(dep + 1, Depth::_2);
        assert_eq!(dep + 2, Depth::_3);
        assert_eq!(dep + 3, Depth::_4);
        assert_eq!(dep + 4, Depth::_5);

        assert!(catch_unwind(|| dep + 5).is_err());

        assert_eq!(Depth::_1 + Depth::_1, Depth::_2);
        assert_eq!(Depth::_1 + Depth::_2, Depth::_3);
        assert_eq!(Depth::_1 + Depth::_3, Depth::_4);
        assert_eq!(Depth::_1 + Depth::_4, Depth::_5);

        assert_eq!(Depth::_2 + Depth::_3, Depth::_5);

        assert!(catch_unwind(|| Depth::_1 + Depth::_1 + Depth::_4).is_err());
    }
}
