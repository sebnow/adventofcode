#[cfg(test)]
#[macro_use]
extern crate quickcheck;

mod stringcircle;
use stringcircle::StringCircle;

fn answer_1(size: usize, input: &[usize]) -> i32 {
    let mut circle = StringCircle::new(size);
    circle.hash(input)
}

fn main() {
    println!("Part 1: {:?}", answer_1(256, &[206, 63, 255, 131, 65, 80, 238, 157, 254, 24, 133, 2, 16, 0, 1, 3]));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1() {
        let input: [usize; 4] = [3, 4, 1, 5];
        assert_eq!(answer_1(5, &input), 12)
    }
}
