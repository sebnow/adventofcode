use std::collections::HashMap;

pub struct Password(Vec<u64>);

impl Password {
    pub fn new(n: u64) -> Self {
        let mut x = n;
        let mut ns = Vec::with_capacity(6);

        loop {
            ns.insert(0, x % 10);
            x /= 10;

            if x == 0 {
                break;
            }
        }

        Password(ns)
    }

    pub fn is_candidate1(&self) -> bool {
        let a = self.0.iter();
        let b = self.0.iter().skip(1);

        let mut has_dup = false;
        for (a, b) in a.zip(b) {
            has_dup = has_dup || a == b;

            if a > b {
                return false;
            }
        }

        has_dup
    }

    pub fn is_candidate2(&self) -> bool {
        let a = self.0.iter();
        let b = self.0.iter().skip(1);
        let mut dupes = HashMap::new();

        for (a, b) in a.zip(b) {
            if a == b {
                let count = dupes.entry(a).or_insert(0);
                *count += 1;
            }

            if a > b {
                return false;
            }
        }

        dupes.values().filter(|&x| *x == 1).count() > 0
    }
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<Password> {
    let mut i = input.split('-').map(|x| x.parse().unwrap());
    let min = i.next().unwrap();
    let max = i.next().unwrap();

    (min..=max).map(Password::new).collect()
}

#[aoc(day4, part1)]
fn answer_1(xs: &[Password]) -> usize {
    xs.iter().filter(|x| x.is_candidate1()).count()
}

#[aoc(day4, part2)]
fn answer_2(xs: &[Password]) -> usize {
    xs.iter().filter(|x| x.is_candidate2()).count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn examples_1() {
        assert_eq!(1, answer_1(&input_generator("111111-111111")));
        assert_eq!(0, answer_1(&input_generator("223450-223450")));
        assert_eq!(0, answer_1(&input_generator("123789-123789")));
        assert_eq!(0, answer_1(&input_generator("123012-123012")));
    }

    #[test]
    fn examples_2() {
        assert_eq!(1, answer_2(&input_generator("112233-112233")));
        assert_eq!(0, answer_2(&input_generator("123444-123444")));
        assert_eq!(1, answer_2(&input_generator("111122-111122")));
    }
}
