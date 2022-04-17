use crate::money::Money;

#[derive(Debug)]
pub(crate) struct Dollar {
    amount: u64,
}

impl Dollar {
    fn new(amount: u64) -> Self {
        Self { amount }
    }
}

impl Money for Dollar {
    fn amount(&self) -> u64 {
        self.amount
    }

    fn times(&self, multiplier: u64) -> Self {
        Self {
            amount: self.amount * multiplier,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::money::dollar::Dollar;
    use crate::money::Money;

    #[test]
    fn test_multiplication() {
        let five = Dollar::new(5);
        assert!(Dollar::new(10).equals(five.times(2)));
        assert!(Dollar::new(15).equals(five.times(3)));
    }

    #[test]
    fn test_equality() {
        assert!(Dollar::new(5).equals(Dollar::new(5)));
        assert!(!Dollar::new(5).equals(Dollar::new(6)));
    }
}
