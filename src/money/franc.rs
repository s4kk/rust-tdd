use crate::money::MoneyTrait;

#[derive(Debug, PartialEq)]
pub(crate) struct Franc {
    pub(crate) amount: u64,
}

impl Franc {
    fn new(amount: u64) -> Self {
        Self { amount }
    }
}

impl MoneyTrait for Franc {
    fn times(&self, multiplier: u64) -> Self {
        Self {
            amount: self.amount * multiplier,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::money::franc::Franc;
    use crate::money::MoneyTrait;

    #[test]
    fn test_multiplication() {
        let five = Franc::new(5);
        assert_eq!(Franc::new(10), five.times(2));
        assert_eq!(Franc::new(15), five.times(3));
    }

    #[test]
    fn test_equality() {
        assert_eq!(Franc::new(5), Franc::new(5));
        assert_ne!(Franc::new(5), Franc::new(6));
    }
}
