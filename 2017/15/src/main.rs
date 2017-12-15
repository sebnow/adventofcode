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
        println!("{:?} * {:}", curr, self.factor);
        self.curr = (curr * self.factor) % DENOMINATOR;

        Some(self.curr)
    }
}

fn main() {
    println!("Hello, world!");
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
}
