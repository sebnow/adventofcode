pub struct DigitIterator {
    x: i64,
}

impl DigitIterator {
    pub fn new(x: i64) -> Self {
        DigitIterator { x }
    }
}

impl Iterator for DigitIterator {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.x == 0 {
            return None;
        }

        let item = self.x % 10;
        self.x /= 10;

        Some(item as u8)
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_digit_iterator() {
        let iter = DigitIterator::new(12_345_678);
        assert_eq!(vec![8, 7, 6, 5, 4, 3, 2, 1], iter.collect::<Vec<u8>>());
    }
}
