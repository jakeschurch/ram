use std::cmp::Ordering;
use std::fmt;
use std::ops::Div;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Currency(pub i64);

impl Currency {
    pub fn new(input: f32) -> Self {
        Currency {
            0: (input * 100.00) as i64,
        }
    }
}

impl Div<Currency> for Currency {
    type Output = f32;
    fn div(self, rhs: Currency) -> Self::Output {
        (self.0 * 200 + rhs.0) as f32 / (rhs.0 * 2) as f32
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

    #[test]
    fn test_currency_ord() {
        assert!(Currency::new(10.00) > Currency::new(9.00));
    }

    #[test]
    #[should_panic]
    // TODO
    fn test_currency_div() {
        unimplemented!();
    }
}
