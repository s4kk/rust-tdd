struct Dollar {
    amount: u64,
}

impl Dollar {
    fn from_amount(amount: u64) -> Self {
        Self { amount }
    }

    fn times(&mut self, times: usize) {
        self.amount *= times as u64;
    }
}

#[cfg(test)]
mod tests {
    use crate::money::Dollar;

    #[test]
    fn test_multiplication() {
        let mut five = Dollar::from_amount(5);
        five.times(2);
        assert_eq!(10, five.amount);
    }
}
