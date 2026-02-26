mod order;
mod tick;
mod unit;

pub mod parse;

pub use crate::order::OrderLike;
pub use crate::tick::TickLike;
pub use crate::unit::*;

/// identifier for market depth of ask/bid
#[repr(u8)]
#[derive(Debug)]
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

#[cfg(test)]
mod test {
    use super::Depth;

    #[test]
    fn depth() {
        assert_eq!(0, Depth::_1 as u8);
        assert_eq!(1, Depth::_2 as u8);
        assert_eq!(2, Depth::_3 as u8);
        assert_eq!(3, Depth::_4 as u8);
        assert_eq!(4, Depth::_5 as u8);
    }
}
