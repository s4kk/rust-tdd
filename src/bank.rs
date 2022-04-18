#![allow(dead_code)]

use crate::expression::Expression;
use crate::money::Currency;
use crate::money::Money;
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct CurrencyPair {
    from: Currency,
    to: Currency,
}

pub struct Bank(HashMap<CurrencyPair, u64>);

impl Bank {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
    pub fn reduce(&self, source: Expression, to: Currency) -> Money {
        source.reduce(self, to)
    }

    pub fn rate(&self, currency: Currency, to: Currency) -> u64 {
        if let Some(&rate) = self.0.get(&CurrencyPair { from: currency, to }) {
            return rate;
        }

        if currency == to {
            return 1;
        };

        unreachable!()
    }

    pub fn add_rate(&mut self, from: Currency, to: Currency, rate: u64) {
        self.0.insert(CurrencyPair { from, to }, rate);
    }
}

#[cfg(test)]
mod tests {
    use crate::bank::Bank;
    use crate::expression::sum::Sum;

    use crate::money::{Currency, Money};

    #[test]
    fn test_reduce_sum() {
        let sum = Sum::new(Money::dollar(3), Money::dollar(4));
        let bank = Bank::new();
        let reduced = bank.reduce(sum, Currency::Dollar);
        assert_eq!(Money::new(7, Currency::Dollar), reduced);
    }

    #[test]
    fn test_reduce_money() {
        let bank = Bank::new();
        let money = bank.reduce(Money::dollar(5), Currency::Dollar);
        assert_eq!(Money::new(5, Currency::Dollar), money);
    }

    #[test]
    fn test_reduce_money_different_currency() {
        let mut bank = Bank::new();
        bank.add_rate(Currency::Franc, Currency::Dollar, 2);
        let money = bank.reduce(Money::franc(2), Currency::Dollar);
        assert_eq!(Money::new(1, Currency::Dollar), money);
    }

    #[test]
    fn test_identity_rate() {
        assert_eq!(1, Bank::new().rate(Currency::Dollar, Currency::Dollar));
    }

    #[test]
    fn test_mixed_addition() {
        let five_bucks = Money::dollar(5);
        let ten_francs = Money::franc(10);
        let mut bank = Bank::new();
        bank.add_rate(Currency::Franc, Currency::Dollar, 2);
        let result = bank.reduce(five_bucks.plus(ten_francs), Currency::Dollar);
        assert_eq!(Money::new(10, Currency::Dollar), result);
    }

    #[test]
    fn test_sum_plus_money() {
        let five_bucks = Money::dollar(5);
        let ten_francs = Money::franc(10);
        let mut bank = Bank::new();
        bank.add_rate(Currency::Franc, Currency::Dollar, 2);
        let sum = Sum::new(five_bucks.clone(), ten_francs).plus(five_bucks.clone());
        let result = bank.reduce(sum, Currency::Dollar);
        assert_eq!(Money::new(15, Currency::Dollar), result);
    }

    #[test]
    fn test_sum_times() {
        let five_bucks = Money::dollar(5);
        let ten_francs = Money::franc(10);
        let mut bank = Bank::new();
        bank.add_rate(Currency::Franc, Currency::Dollar, 2);
        let sum = Sum::new(five_bucks.clone(), ten_francs).times(2);
        let result = bank.reduce(sum, Currency::Dollar);
        assert_eq!(Money::new(20, Currency::Dollar), result);
    }
}
