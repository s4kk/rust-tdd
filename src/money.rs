use crate::money::dollar::Dollar;
use crate::money::franc::Franc;

mod dollar;
mod franc;

pub(crate) trait MoneyTrait {
    fn times(&self, multiplier: u64) -> Self;
}

enum Money {
    Dollar(Dollar),
    Franc(Franc),
}

impl Money {
    fn amount(&self) -> u64 {
        use Money::*;
        match self {
            Dollar(dollar) => dollar.amount,
            Franc(franc) => franc.amount,
        }
    }
}

impl PartialEq for Money {
    fn eq(&self, other: &Self) -> bool {
        use Money::*;
        match (self, other) {
            (Dollar(dollar), Dollar(other)) => dollar == other,
            (Franc(franc), Franc(other)) => franc == other,
            _ => false,
        }
    }
}
