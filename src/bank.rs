use crate::expression::Expression;
use crate::money::Currency;
use crate::money::Money;

pub(crate) struct Bank;

impl Bank {
    pub(crate) fn new() -> Self {
        Self
    }
    pub(crate) fn reduce(&self, source: Expression, to: Currency) -> Money {
        source.reduce(to)
    }
}
