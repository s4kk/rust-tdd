use crate::expression::sum::Sum;
use crate::money::{Currency, Money};

pub(crate) mod sum;

pub(crate) enum Expression {
    Money(Money),
    Sum(Sum),
}

impl Expression {
    pub(crate) fn reduce(&self, to: Currency) -> Money {
        match self {
            Expression::Money(money) => money.reduce(to),
            Expression::Sum(sum) => sum.reduce(to),
        }
    }
}
