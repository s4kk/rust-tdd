use crate::bank::Bank;
use crate::expression::Expression;
use crate::money::{Currency, Money};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Sum {
    pub(crate) augend: Expression,
    pub(crate) addend: Expression,
}

impl Sum {
    pub(crate) fn new(augend: Expression, addend: Expression) -> Expression {
        Expression::Sum(Box::new(Self { augend, addend }))
    }

    pub(crate) fn reduce(&self, bank: &Bank, to: Currency) -> Money {
        Money::new(
            self.augend.reduce(bank, to).amount + self.addend.reduce(bank, to).amount,
            to,
        )
    }
}
