use crate::bank::Bank;
use crate::expression::sum::Sum;
use crate::money::{Currency, Money};

pub(crate) mod sum;

pub(crate) enum Expression {
    Money(Money),
    Sum(Sum),
}

impl Expression {
    pub(crate) fn reduce(&self, bank: &Bank, to: Currency) -> Money {
        match self {
            Expression::Money(money) => money.reduce(bank, to),
            Expression::Sum(sum) => sum.reduce(bank, to),
        }
    }
}
