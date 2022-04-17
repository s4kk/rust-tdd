use crate::expression::Expression;
use crate::money::{Currency, Money};

pub struct Sum {
    pub(crate) augend: Money,
    pub(crate) addend: Money,
}

impl Sum {
    pub(crate) fn new(augend: Money, addend: Money) -> Expression {
        Expression::Sum(Self { augend, addend })
    }

    pub(crate) fn reduce(&self, to: Currency) -> Money {
        Money::new(self.augend.amount + self.addend.amount, to)
    }
}
