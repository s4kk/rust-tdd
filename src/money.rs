#[derive(Debug)]
struct Money {
    amount: u64,
    currency: Currency,
}

#[derive(Debug, Clone, Copy)]
enum Currency {
    Dollar,
    Franc,
}

impl Money {
    fn new(amount: u64, currency: Currency) -> Self {
        Money { amount, currency }
    }
}

impl Money {
    fn amount(&self) -> u64 {
        self.amount
    }

    fn times(&self, multiplier: u64) -> Self {
        Self {
            amount: self.amount * multiplier,
            currency: self.currency,
        }
    }

    fn currency(&self) -> &'static str {
        use Currency::*;
        match self.currency {
            Dollar => "USD",
            Franc => "CHF",
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
    use crate::money::{Currency, Money};

    #[test]
    fn test_multiplication() {
        let five = Money::new(5, Currency::Dollar);
        assert_eq!(Money::new(10, Currency::Dollar), five.times(2));
        assert_eq!(Money::new(15, Currency::Dollar), five.times(3));
    }

    #[test]
    fn test_equality() {
        assert_ne!(
            Money::new(5, Currency::Dollar),
            Money::new(5, Currency::Franc)
        );
        assert_eq!(
            Money::new(5, Currency::Dollar),
            Money::new(5, Currency::Dollar)
        );
        assert_ne!(
            Money::new(5, Currency::Dollar),
            Money::new(6, Currency::Dollar)
        );
    }

    #[test]
    fn test_currency() {
        assert_eq!("USD", Money::new(1, Currency::Dollar).currency());
        assert_eq!("CHF", Money::new(1, Currency::Franc).currency());
    }
}
