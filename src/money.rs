#[derive(Debug)]
struct Dollar {
    amount: u64,
}

impl Dollar {
    fn from_amount(amount: u64) -> Self {
        Self { amount }
    }

    fn times(&self, times: u64) -> Self {
        Self {
            amount: self.amount * times,
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
        assert_eq!(Dollar::from_amount(10), five.times(2));
        assert_eq!(Dollar::from_amount(15), five.times(3));
    }

    #[test]
    fn test_equality() {
        assert_eq!(Dollar::from_amount(5), Dollar::from_amount(5));
        assert_ne!(Dollar::from_amount(5), Dollar::from_amount(6));
    }
}
