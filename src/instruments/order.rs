use chrono::{DateTime, Utc};
use instruments::Currency;
use std::cell::RefCell;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub struct Order {
    // REVIEW: id field?
    pub symbol: &'static str,
    pub price: Currency,
    pub qty: u32,
    pub side: Side,
    pub datetime: DateTime<Utc>,
    state: State,
}

impl Order {
    pub fn new(
        symbol: &'static str,
        price: f32,
        qty: u32,
        side: Side,
        datetime: DateTime<Utc>,
    ) -> Order {
        Order {
            symbol,
            price: Currency::new(price),
            qty,
            side,
            datetime,
            state: State::New,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, PartialOrd, Ord)]
/// State indicates the state machine of an order.
pub enum State {
    New,
    PartiallyFilled,
    Filled,
    Cancelled,
    Suspended,
    Expired,
    Rejected,
}

#[derive(PartialEq, Eq, Debug, Clone, PartialOrd, Ord)]
/// Side indicates an order's intention to either buy or sell shares.  
pub enum Side {
    Buy,
    Sell,
}

#[derive(PartialEq, Eq, Debug, Clone, PartialOrd, Ord)]
/// Logic indicates an order's execution logic.
/// NOTE(Logic): Currently only allowing market logic orders.
pub enum Logic {
    Market,
}
