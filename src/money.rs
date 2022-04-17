mod dollar;
mod franc;

pub(crate) trait Money {
    fn amount(&self) -> u64;
    fn times(&self, multiplier: u64) -> Self;
    fn equals<M: Money>(&self, other: M) -> bool {
        self.amount() == other.amount()
    }
}
