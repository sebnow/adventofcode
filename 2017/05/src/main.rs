#[derive(Debug, Eq, PartialEq)]
struct Answer {
    steps: i32,
}

fn answer_with_input(input: &[i32]) -> Answer {
    Answer{steps: 0}
}

fn answer() -> Answer {
    let input = [];
    answer_with_input(&input)
}

fn main() {
    println!("{:?}", answer());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(answer_with_input(&[0, 3, 0, 1, -3]), Answer{steps: 5});
    }
}
