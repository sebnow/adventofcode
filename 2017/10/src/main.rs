#[cfg(test)]
#[macro_use]
extern crate quickcheck;

mod stringcircle;
use stringcircle::StringCircle;

fn answer_1(size: usize, input: &[usize]) -> i32 {
    let mut circle = StringCircle::new(size);
    circle.hash(input)
}

fn answer_2(input: &str) -> String {
    let mut lengths: Vec<usize> = input.as_bytes()
        .to_vec()
        .iter()
        .map(|x| *x as usize)
        .collect();
    lengths.extend_from_slice(&[17, 31, 73, 47, 23]);

    let mut circle = StringCircle::new();
    for _ in 0..64 {
        circle.hash(&lengths);
    }

    String::from("")
}

fn main() {
    println!("Part 1: {:?}", answer_1(256, &[206, 63, 255, 131, 65, 80, 238, 157, 254, 24, 133, 2, 16, 0, 1, 3]));
    println!("Part 2: {:?}", answer_2("206,63,255,131,65,80,238,157,254,24,133,2,16,0,1,3"));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(answer_1(5, &[3, 4, 1, 5]), 12)
    }

    #[test]
    fn example2() {
        assert_eq!(answer_2(""), String::from("a2582a3a0e66e6e86e3812dcb672a272"));
        assert_eq!(answer_2("AoC 2017"), String::from("33efeb34ea91902bb2f59c9920caa6cd"));
        assert_eq!(answer_2("1,2,3"), String::from("3efbe78a8d82f29979031a4aa0b16a9d"));
        assert_eq!(answer_2("1,2,4"), String::from("63960835bcdc130f0b66d7ff4f6a5a8e"));
    }
}
