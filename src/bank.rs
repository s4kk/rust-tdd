use crate::expression::Expression;
use crate::money::Currency;
use crate::money::Money;
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Hash)]
pub(crate) struct CurrencyPair {
    from: Currency,
    to: Currency,
}

pub(crate) struct Bank(HashMap<CurrencyPair, u64>);

impl Bank {
    pub(crate) fn new() -> Self {
        Self(HashMap::new())
    }
    pub(crate) fn reduce(&self, source: Expression, to: Currency) -> Money {
        source.reduce(self, to)
    }

    pub(crate) fn rate(&self, currency: Currency, to: Currency) -> u64 {
        if let Some(rate) = self.0.get(&CurrencyPair { from: currency, to }) {
            return *rate;
        }

        if currency == to {
            return 1;
        };

        unreachable!()
    }

    pub(crate) fn addRate(&mut self, from: Currency, to: Currency, rate: u64) {
        self.0.insert(CurrencyPair { from, to }, rate);
    }
}

#[cfg(test)]
mod tests {
    use crate::bank::Bank;
    use crate::expression::sum::Sum;
    use crate::expression::Expression;
    use crate::money::{Currency, Money};

    #[test]
    fn test_reduce_sum() {
        let sum = Sum::new(Money::dollar(3), Money::dollar(4));
        let bank = Bank::new();
        let reduced = bank.reduce(sum, Currency::Dollar);
        assert_eq!(Money::dollar(7), reduced);
    }

    #[test]
    fn test_reduce_money() {
        let bank = Bank::new();
        let money = bank.reduce(Expression::Money(Money::dollar(5)), Currency::Dollar);
        assert_eq!(money, Money::dollar(5));
    }

    #[test]
    fn test_reduce_money_different_currency() {
        let mut bank = Bank::new();
        bank.addRate(Currency::Franc, Currency::Dollar, 2);
        let money = bank.reduce(Expression::Money(Money::franc(2)), Currency::Dollar);
        assert_eq!(Money::dollar(1), money);
    }

    #[test]
    fn test_identity_rate() {
        assert_eq!(1, Bank::new().rate(Currency::Dollar, Currency::Dollar));
    }
}
