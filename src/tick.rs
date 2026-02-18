use std::ops::{Add, Div, Mul, Sub};

pub trait TicLike<F, T>
where
    T: Add + Mul + Div + Sub + PartialEq + PartialOrd + Into<f64>,
{
    fn last_price(&self) -> F;

    fn volume(&self) -> T;

    fn open_interest(&self) -> T;

    fn bid_price(&self, index: usize) -> F;

    fn ask_price(&self, index: usize) -> F;

    fn bid_volume(&self, index: usize) -> T;

    fn ask_volume(&self, index: usize) -> T;

    fn mid_price(&self) -> F;

    fn turnover(&self) -> F;

    fn hms(&self, base_time: u32) -> (u32, u32, u32);

    fn timestamp(&self, base_time: u32) -> u32;

    fn snaptime(&self) -> u32;

    fn ms(&self) -> u16;

    fn code(&self) -> u64;

    fn ask_volume_all(&self) -> T;

    fn bid_volume_all(&self) -> T;
}
