use crate::money::dollar::Dollar;
use crate::money::franc::Franc;

mod dollar;
mod franc;

#[derive(Debug)]
enum Money {
    Dollar(Dollar),
    Franc(Franc),
}

impl Money {
    fn new_dollar(amount: u64) -> Self {
        Money::Dollar(Dollar::new(amount))
    }

    fn new_franc(amount: u64) -> Self {
        Money::Franc(Franc::new(amount))
    }
}

impl Money {
    fn amount(&self) -> u64 {
        use Money::*;
        match self {
            Dollar(dollar) => dollar.amount,
            Franc(franc) => franc.amount,
        }
    }

    fn times(&self, multiplier: u64) -> Money {
        use Money::*;
        match self {
            Dollar(dollar) => Dollar(dollar.times(multiplier)),
            Franc(franc) => Franc(franc.times(multiplier)),
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

#[cfg(test)]
mod tests {
    use crate::money::Money;

    #[test]
    fn test_equality() {
        assert_ne!(Money::new_dollar(5), Money::new_franc(5));
        assert_eq!(Money::new_dollar(5), Money::new_dollar(5));
        assert_ne!(Money::new_dollar(5), Money::new_dollar(6));
        assert_eq!(Money::new_franc(5), Money::new_franc(5));
        assert_ne!(Money::new_franc(5), Money::new_franc(6));
    }
}
