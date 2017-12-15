const DENOMINATOR: u32 = 2147483647;

struct Generator {
    factor: u32,
    curr: u32,
}

impl Generator {
    pub fn new(factor: u32, init: u32) -> Self {
        Generator {
            factor: factor,
            curr: init,
        }
    }
}

impl Iterator for Generator {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let curr = self.curr;
        self.curr = ((curr as u64 * self.factor as u64) % DENOMINATOR as u64) as u32;

        Some(self.curr)
    }
}

#[inline]
fn judge(a: u32, b: u32) -> bool {
    let mask: u32 = (2 as u32).pow(16) - 1;
    (a & mask) == (b & mask)
}

fn answer_1(a_seed: u32, b_seed: u32) -> usize {
    let pairs = 40 * 1000 * 1000;
    let a = Generator::new(16807, a_seed);
    let b = Generator::new(48271, b_seed);

    a.take(pairs)
        .zip(b.take(pairs))
        .filter(|&(a, b)| judge(a, b))
        .count()
}

fn main() {
    println!("Part 1: {}", answer_1(618, 814));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_generator() {
        let a = Generator::new(16807, 65);
        let b = Generator::new(48271, 8921);

        assert_eq!(
            vec![1092455, 1181022009, 245556042, 1744312007, 1352636452],
            a.take(5).collect::<Vec<u32>>()
        );
        assert_eq!(
            vec![430625591, 1233683848, 1431495498, 137874439, 285222916],
            b.take(5).collect::<Vec<u32>>()
        );
    }

    #[test]
    fn test_judge() {
        assert_eq!(false, judge(245556042, 430625591));
        assert_eq!(false, judge(1181022009, 1233683848));
        assert_eq!(true, judge(245556042, 1431495498));
        assert_eq!(false, judge(1744312007, 137874439));
        assert_eq!(false, judge(1352636452, 285222916));
    }

    #[test]
    fn test_answer_1() {
        assert_eq!(588, answer_1(65, 8921));
    }
}
