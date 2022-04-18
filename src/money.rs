use crate::bank::Bank;
use crate::expression::sum::Sum;
use crate::expression::Expression;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Money {
    pub(crate) amount: u64,
    pub(crate) currency: Currency,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub(crate) enum Currency {
    Dollar,
    Franc,
}

impl Money {
    pub(crate) fn franc(amount: u64) -> Expression {
        Expression::Money(Self::new(amount, Currency::Franc))
    }

    pub(crate) fn dollar(amount: u64) -> Expression {
        Expression::Money(Self::new(amount, Currency::Dollar))
    }

    pub(crate) fn new(amount: u64, currency: Currency) -> Self {
        Self { amount, currency }
    }
}

impl Money {
    fn amount(&self) -> u64 {
        self.amount
    }

    pub(crate) fn times(&self, multiplier: u64) -> Expression {
        Expression::Money(Self {
            amount: self.amount * multiplier,
            currency: self.currency,
        })
    }

    pub(crate) fn currency(&self) -> &'static str {
        use Currency::*;
        match self.currency {
            Dollar => "USD",
            Franc => "CHF",
        }
    }

    pub(crate) fn plus(self, other: Expression) -> Expression {
        Sum::new(Expression::Money(self), other)
    }

    pub(crate) fn reduce(self, bank: &Bank, to: Currency) -> Money {
        let rate = bank.rate(self.currency, to);
        Self::new(self.amount / rate, to)
    }
}

impl PartialEq for Money {
    fn eq(&self, other: &Self) -> bool {
        self.amount() == other.amount() && self.currency() == other.currency()
    }
}

impl Eq for Money {}

#[cfg(test)]
mod tests {
    use crate::bank::Bank;

    use crate::expression::Expression;
    use crate::money::{Currency, Money};

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
        let sum = five.plus(five.clone());
        let bank = Bank::new();
        let reduced = bank.reduce(sum, Currency::Dollar);
        assert_eq!(Money::new(10, Currency::Dollar), reduced);
    }

    #[test]
    fn test_plus_method_return_sum() {
        let five = Money::dollar(5);
        let expr = five.plus(five.clone());
        let sum = match expr {
            Expression::Sum(sum) => sum,
            _ => unreachable!(),
        };
        assert_eq!(five, sum.augend);
        assert_eq!(five, sum.addend);
    }
}
