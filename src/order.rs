use crate::unit::{Direction, Exchange, Offset, OrderType, Status};

pub trait OrderLike {
    type Int;
    type Float;

    fn order_type(&self) -> OrderType;

    fn symbol(&self) -> u64;

    fn exchange(&self) -> Exchange;

    fn token(&self) -> u32;

    fn order_id(&self) -> u64;

    fn direction(&self) -> Direction;

    fn offset(&self) -> Offset;

    fn price(&self) -> Self::Float;

    fn volume(&self) -> Self::Int;

    fn status(&self) -> Status;
}
