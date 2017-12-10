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
    println!("Hello, world!");
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
