use crate::money::Money;

#[derive(Debug)]
pub(crate) struct Franc {
    amount: u64,
}

impl Franc {
    fn new(amount: u64) -> Self {
        Self { amount }
    }
}

impl Money for Franc {
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
    use crate::money::franc::Franc;
    use crate::money::Money;

    #[test]
    fn test_multiplication() {
        let five = Franc::new(5);
        assert!(Franc::new(10).equals(five.times(2)));
        assert!(Franc::new(15).equals(five.times(3)));
    }

    #[test]
    fn test_equality() {
        assert!(Franc::new(5).equals(Franc::new(5)));
        assert!(!Franc::new(5).equals(Franc::new(6)));
    }
}
