use crate::{Direction, Exchange, Offset, OrderType, Status};
use std::ops::{Add, Div, Mul, Sub};
pub trait FutOrder<F, T>
where
    T: Add + Mul + Div + Sub + PartialEq + PartialOrd + Into<f64>,
{
    fn order_type(&self) -> OrderType;
    fn symbol(&self) -> u64;
    fn exchange(&self) -> Exchange;
    fn token(&self) -> u32;
    fn order_id(&self) -> u64;
    fn direction(&self) -> Direction;
    fn offset(&self) -> Offset;
    fn price(&self) -> F;
    fn volume(&self) -> T;
    fn status(&self) -> Status;
}
