use crate::bank::Bank;
use crate::expression::Expression;
use crate::money::{Currency, Money};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Sum {
    pub augend: Expression,
    pub addend: Expression,
}

impl Sum {
    pub fn new(augend: Expression, addend: Expression) -> Expression {
        Expression::Sum(Box::new(Self { augend, addend }))
    }

    pub fn reduce(&self, bank: &Bank, to: Currency) -> Money {
        Money::new(
            self.augend.reduce(bank, to).amount + self.addend.reduce(bank, to).amount,
            to,
        )
    }

    pub fn plus(&self, other: Expression) -> Expression {
        Self::new(Expression::Sum(Box::new(self.clone())), other)
    }

    pub fn times(&self, multiplier: u64) -> Expression {
        Self::new(self.augend.times(multiplier), self.addend.times(multiplier))
    }
}
