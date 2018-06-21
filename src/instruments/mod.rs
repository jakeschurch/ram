pub use self::currency::Currency;
pub use self::order::{Order, Side};

mod currency;
mod order;

use chrono::{DateTime, Utc};

// TODO: Buy/Sell Error types.

// REVIEW:
// enum _Asset {
//     Quote,
//     Order
// }

trait Instrument: Sized {
    fn buy(order: Order) -> Result<Self, ()>;
    fn sell(&mut self, v: usize) -> Result<Option<Self>, ()>;
}

pub struct _Quote {
    pub symbol: &'static str,
    pub qty: u32,
    pub price: Currency,
    pub datetime: DateTime<Utc>,
}
