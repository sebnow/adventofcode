#[derive(Debug, Eq, PartialEq)]
struct Answer {
    steps: i32,
}

fn answer(input: i32) -> Answer {
    Answer{steps: 0}
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(answer(1), Answer{steps: 0});
        assert_eq!(answer(12), Answer{steps: 3});
        assert_eq!(answer(23), Answer{steps: 2});
        assert_eq!(answer(1024), Answer{steps: 31});
    }
}
