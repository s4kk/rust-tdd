#[derive(Debug, PartialEq)]
struct Dollar {
    amount: u64,
}

impl Dollar {
    fn new(amount: u64) -> Self {
        Self { amount }
    }

    fn times(&self, multiplier: u64) -> Self {
        Self {
            amount: self.amount * multiplier,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::money::Dollar;

    #[test]
    fn test_multiplication() {
        let five = Dollar::new(5);
        assert_eq!(Dollar::new(10), five.times(2));
        assert_eq!(Dollar::new(15), five.times(3));
    }

    #[test]
    fn test_equality() {
        assert_eq!(Dollar::new(5), Dollar::new(5));
        assert_ne!(Dollar::new(5), Dollar::new(6));
    }
}
