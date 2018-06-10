extern crate chrono;

use std::{cmp::Ordering, fmt};

#[derive(Debug, PartialEq, Eq)]
pub struct Currency {
    amount: i64,
}

impl Currency {
    pub fn new(input: f64) -> Currency {
        Currency {
            amount: (input * 100.00) as i64,
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
        self.amount.cmp(&other.amount)
    }
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s: String = self.amount.to_string();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_currency_new() {
        assert_eq!(Currency { amount: 1025 }, Currency::new(10.25));
        assert_eq!(Currency { amount: -1025 }, Currency::new(-10.25));
    }
    #[test]
    fn test_currency_fmt() {
        println!("{}", Currency::new(10.25));
        println!("{}", Currency::new(1025.00));
    }
}
