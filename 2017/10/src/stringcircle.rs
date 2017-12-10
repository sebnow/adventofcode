use std::fmt;

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
        debug_assert!(self.knots.len() > 2);

        for length in input {
            let pos = self.pos;
            circular_reverse(&mut self.knots, pos, pos + *length);
            self.pos += *length + self.skip_size;
            self.skip_size += 1;
        }

        self.knots[0] + self.knots[1]
    }
}

fn circular_reverse<T: fmt::Debug>(slice: &mut [T], start: usize, end: usize) -> &[T] {
    for cur in start..((end - start) / 2) + 1 {
        let end_cur = end - (cur - start);
        slice.swap(cur, end_cur);
    }

    slice
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn hash_with_empty_input_is_range() {
        assert_eq!(StringCircle::new(10).hash(&[]), 1);
    }

    #[test]
    fn circular_reverse_with_single_element_does_nothing() {
        assert_eq!(circular_reverse(&mut [1], 0, 0), [1]);
    }

    #[test]
    fn circular_reverse_extended() {
        assert_eq!(circular_reverse(&mut [1, 2, 3, 4], 2, 5), [1, 3, 2, 4]);
    }

    quickcheck! {
        fn circurlar_reverse_full(xs: Vec<i32>) -> bool {
            let len = xs.len();
            let mut rev = xs.clone();
            let mut xs = xs;
            rev.reverse();
            rev == circular_reverse(xs.as_mut_slice(), 0, len - 1)
        }
    }
}
