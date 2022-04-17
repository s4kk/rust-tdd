trait Expression {}

impl Expression for Money {}

struct Bank;

impl Bank {
    fn new() -> Self {
        Self
    }
    fn reduce<E: Expression>(&self, _source: E, _to: Currency) -> Money {
        Money::dollar(10)
    }
}

#[derive(Debug, Clone, Copy)]
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
    fn franc(amount: u64) -> Self {
        Money {
            amount,
            currency: Currency::Franc,
        }
    }

    fn dollar(amount: u64) -> Self {
        Money {
            amount,
            currency: Currency::Dollar,
        }
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

    fn plus(&self, other: Money) -> impl Expression {
        Self {
            amount: self.amount() + other.amount(),
            currency: self.currency,
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
    use crate::money::{Bank, Currency, Money};

    #[test]
    fn test_multiplication() {
        let five = Money::dollar(5);
        assert_eq!(Money::dollar(10), five.times(2));
        assert_eq!(Money::dollar(15), five.times(3));
    }

    #[test]
    fn test_equality() {
        assert_ne!(Money::dollar(5), Money::franc(5));
        assert_eq!(Money::dollar(5), Money::dollar(5));
        assert_ne!(Money::dollar(5), Money::dollar(6));
    }

    #[test]
    fn test_currency() {
        assert_eq!("USD", Money::dollar(1).currency());
        assert_eq!("CHF", Money::franc(1).currency());
    }

    #[test]
    fn test_simple_addition() {
        let five = Money::dollar(5);
        let sum = five.plus(five);
        let bank = Bank::new();
        let reduced = bank.reduce(sum, Currency::Dollar);
        assert_eq!(Money::dollar(10), reduced);
    }
}
