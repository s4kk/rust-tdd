#[derive(Debug)]
struct Dollar {
    amount: u64,
}

impl Dollar {
    fn from_amount(amount: u64) -> Self {
        Self { amount }
    }

    fn times(&self, times: usize) -> Self {
        Self {
            amount: self.amount * times as u64,
        }
    }
}

impl PartialEq for Dollar {
    fn eq(&self, other: &Self) -> bool {
        self.amount == other.amount
    }
}

#[cfg(test)]
mod tests {
    use crate::money::Dollar;

    #[test]
    fn test_multiplication() {
        let five = Dollar::from_amount(5);
        let product = five.times(2);
        assert_eq!(10, product.amount);
        let product = five.times(3);
        assert_eq!(15, product.amount);
    }

    #[test]
    fn test_equality() {
        assert_eq!(Dollar::from_amount(5), Dollar::from_amount(5));
        assert_ne!(Dollar::from_amount(5), Dollar::from_amount(6));
    }
}
