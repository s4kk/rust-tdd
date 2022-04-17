use crate::bank::Bank;
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

    pub(crate) fn reduce(&self, bank: &Bank, to: Currency) -> Money {
        Money::new(
            self.augend.reduce(bank, to).amount + self.addend.reduce(bank, to).amount,
            to,
        )
    }
}
