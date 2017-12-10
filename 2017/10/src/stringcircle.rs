use std::fmt;

#[derive(Debug, Eq, PartialEq)]
pub struct StringCircle {
    knots: Vec<i32>,
    pos: usize,
    skip_size: usize,
}

impl StringCircle {
    pub fn new(size: usize) -> Self {
        StringCircle {
            knots: (0..(size as i32)).collect(),
            pos: 0,
            skip_size: 0,
        }
    }

    pub fn hash(&mut self, input: &[usize]) -> i32 {
        let knotlen = self.knots.len();
        debug_assert!(knotlen > 2);

        for length in input {
            let pos = self.pos;
            circular_reverse(&mut self.knots, pos, pos + *length - 1);
            self.pos = position_looped(knotlen, self.pos + *length + self.skip_size);
            self.skip_size += 1;
        }

        self.knots[0] * self.knots[1]
    }
}

fn circular_reverse<T: fmt::Debug>(slice: &mut [T], start: usize, end: usize) -> &[T] {
    let len = slice.len();
    if len <= 1 {
        return slice
    }

    for cur in start..middle(start, end) {
        let end_cur = end - (cur - start);
        slice.swap(position_looped(len, cur), position_looped(len, end_cur));
    }

    slice
}

#[inline]
fn middle(a: usize, b: usize) -> usize {
    a + ((b as f64 - a as f64) / 2.0).ceil() as usize
}

#[inline]
fn position_looped(len: usize, pos: usize) -> usize {
    match len {
        0 => 0,
        1 => 0,
        _ => pos % len,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use quickcheck::TestResult;

    #[test]
    fn hash() {
        let mut string = StringCircle::new(5);
        assert_eq!(string.hash(&[3, 4, 1, 5]), 12);
    }

    #[test]
    fn circular_reverse_with_single_element_does_nothing() {
        assert_eq!(circular_reverse(&mut [1], 0, 0), [1]);
    }

    #[test]
    fn circular_reverse_extended() {
        assert_eq!(circular_reverse(&mut [0, 1, 2, 3, 4], 0, 2), [2, 1, 0, 3, 4]);
        assert_eq!(circular_reverse(&mut [2, 1, 0, 3, 4], 3, 6), [4, 3, 0, 1, 2]);
        assert_eq!(circular_reverse(&mut [4, 3, 0, 1, 2], 1, 5), [3, 4, 2, 1, 0]);
        assert_eq!(circular_reverse(&mut [0, 1, 2, 3, 4], 0, 2), [2, 1, 0, 3, 4]);

        assert_eq!(circular_reverse(&mut [4, 0, 2, 1, 3, 5], 5, 10), [1, 2, 0, 4, 5, 3]);

    }

    #[test]
    fn position_looped_loops() {
        assert_eq!(position_looped(10, 9), 9);
        assert_eq!(position_looped(10, 10), 0);
        assert_eq!(position_looped(10, 11), 1);
    }

    quickcheck! {
        fn circurlar_reverse_full(xs: Vec<i32>) -> TestResult {
            let len = xs.len();
            if len == 0 {
                return TestResult::discard()
            }
            let mut rev = xs.clone();
            let mut xs = xs;
            rev.reverse();
            TestResult::from_bool(rev == circular_reverse(xs.as_mut_slice(), 0, len - 1))
        }

        fn position_looped_never_oob(len: usize, idx: usize) -> TestResult {
            if len == 0 {
                return TestResult::discard()
            }

            TestResult::from_bool(position_looped(len, idx) < len)
        }
    }
}
