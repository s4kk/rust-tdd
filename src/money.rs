mod dollar;
mod franc;

#[derive(Debug)]
enum Money {
    Dollar { amount: u64 },
    Franc { amount: u64 },
}

impl Money {
    fn new_dollar(amount: u64) -> Self {
        Money::Dollar { amount }
    }

    fn new_franc(amount: u64) -> Self {
        Money::Franc { amount }
    }
}

impl Money {
    fn amount(&self) -> u64 {
        use Money::*;
        match self {
            Dollar { amount } => *amount,
            Franc { amount } => *amount,
        }
    }

    fn times(&self, multiplier: u64) -> Money {
        use Money::*;
        match self {
            Dollar { amount } => Dollar {
                amount: amount * multiplier,
            },
            Franc { amount } => Franc {
                amount: amount * multiplier,
            },
        }
    }

    fn currency(&self) -> &'static str {
        use Money::*;
        match self {
            Dollar { .. } => "USD",
            Franc { .. } => "CHF",
        }
    }
}

impl PartialEq for Money {
    fn eq(&self, other: &Self) -> bool {
        self.amount() == other.amount() && self.currency() == other.currency()
    }
}

#[cfg(test)]
mod tests {
    use crate::money::Money;

    #[test]
    fn test_equality() {
        assert_ne!(Money::new_dollar(5), Money::new_franc(5));
        assert_eq!(Money::new_dollar(5), Money::new_dollar(5));
        assert_ne!(Money::new_dollar(5), Money::new_dollar(6));
        assert_eq!(Money::new_franc(5), Money::new_franc(5));
        assert_ne!(Money::new_franc(5), Money::new_franc(6));
    }

    #[test]
    fn test_currency() {
        assert_eq!("USD", Money::new_dollar(1).currency());
        assert_eq!("CHF", Money::new_franc(1).currency());
    }
}
