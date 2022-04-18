use crate::bank::Bank;
use crate::expression::sum::Sum;
use crate::money::{Currency, Money};

pub(crate) mod sum;

#[derive(Debug, Eq, PartialEq, Clone)]
pub(crate) enum Expression {
    Money(Money),
    Sum(Box<Sum>),
}

impl Expression {
    pub(crate) fn reduce(&self, bank: &Bank, to: Currency) -> Money {
        match self {
            Expression::Money(money) => money.reduce(bank, to),
            Expression::Sum(sum) => sum.reduce(bank, to),
        }
    }

    pub(crate) fn plus(&self, addend: Expression) -> Expression {
        match self {
            Expression::Money(money) => money.plus(addend),
            Expression::Sum(sum) => sum.addend.plus(sum.augend.clone()).plus(addend),
        }
    }

    pub(crate) fn currency(&self) -> &'static str {
        match self {
            Expression::Money(money) => money.currency(),
            Expression::Sum(_sum) => unreachable!(),
        }
    }

    pub(crate) fn times(&self, multiplier: u64) -> Expression {
        match self {
            Expression::Money(money) => money.times(multiplier),
            Expression::Sum(_sum) => unreachable!(),
        }
    }
}
