pub mod order;

use chrono::{DateTime, Utc};
use instruments::order::Order;
use std::{cmp::Ordering, fmt};

// TODO: Buy/Sell Error types.

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Currency(i64);

impl Currency {
    pub fn new(input: f64) -> Currency {
        Currency {
            0: (input * 100.00) as i64,
        }
    }
}

impl PartialOrd for Currency {
    fn partial_cmp(&self, other: &Currency) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Currency {
    fn cmp(&self, other: &Currency) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s: String = self.0.to_string();
        let mut result = String::with_capacity(s.len());

        let mut i = 0;
        for c in s.chars() {
            if i == s.len() - 2 {
                result.push('.');
            }
            result.push(c);
            i += 1;
        }

        write!(f, "${}", result)
    }
}
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
    pub qty: usize,
    pub price: Currency,
    pub datetime: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_currency_new() {
        assert_eq!(Currency { 0: 1025 }, Currency::new(10.25));
        assert_eq!(Currency { 0: -1025 }, Currency::new(-10.25));
    }
    #[test]
    fn test_currency_fmt() {
        println!("{}", Currency::new(10.25));
        println!("{}", Currency::new(1025.00));
    }
}
