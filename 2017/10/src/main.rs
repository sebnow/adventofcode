fn answer_1(string: &mut [i32], input: &[i32]) -> i32 {
    0
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1() {
        let mut string: [i32; 5] = [0, 1, 2, 3, 4];
        let input: [i32; 4] = [3, 4, 1, 5];
        assert_eq!(answer_1(&mut string, &input), 12)
    }
}
