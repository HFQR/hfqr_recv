use core::ops::Add;

use crate::Depth;

/// generic abstraction for tick data
pub trait TickLike {
    type Int: Add<Output = Self::Int>;
    type Float;

    fn last_price(&self) -> Self::Float;

    fn turn_over(&self) -> Self::Float;

    fn volume(&self) -> Self::Int;

    fn open_interest(&self) -> Self::Int;

    fn bid_price(&self, dep: Depth) -> Self::Float;

    fn ask_price(&self, dep: Depth) -> Self::Float;

    fn bid_volume(&self, dep: Depth) -> Self::Int;

    fn ask_volume(&self, dep: Depth) -> Self::Int;

    fn snap_time(&self) -> u32;

    fn ms(&self) -> u16;

    fn code(&self) -> u64;

    fn mid_price(&self) -> Self::Float;

    #[inline]
    fn hms(&self, base_time: u32) -> (u32, u32, u32) {
        let time_delta = self.snap_time() - base_time;
        let hour = time_delta / 3600;
        let minute = time_delta % 3600 / 60;
        let second = time_delta % 60;
        (hour, minute, second)
    }

    #[inline]
    fn timestamp(&self, base_time: u32) -> u32 {
        let (h, m, s) = self.hms(base_time);
        let hour = if h <= 2 { 24 + h } else { h };
        hour * 3600 + m * 60 + s
    }

    #[inline]
    fn ask_volume_all(&self) -> Self::Int {
        self.ask_volume(Depth::_1)
            + self.ask_volume(Depth::_2)
            + self.ask_volume(Depth::_3)
            + self.ask_volume(Depth::_4)
            + self.ask_volume(Depth::_5)
    }

    #[inline]
    fn bid_volume_all(&self) -> Self::Int {
        self.bid_volume(Depth::_1)
            + self.bid_volume(Depth::_2)
            + self.bid_volume(Depth::_3)
            + self.bid_volume(Depth::_4)
            + self.bid_volume(Depth::_5)
    }
}
