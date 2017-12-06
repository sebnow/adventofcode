#[derive(Debug, Eq, PartialEq)]
struct Answer {
    steps: i32,
}

fn answer_with_input(input: &mut [i32]) -> Answer {
    let length = input.len();

    let mut steps = 0;
    let mut pos: i32 = 0;

    while 0 <= pos && pos < (length as i32) {
        let idx = pos as usize;
        let jump = input[idx];

        input[idx] += 1;
        pos += jump;
        steps += 1;
    }

    Answer{steps: steps}
}

fn answer() -> Answer {
    let mut input = [];
    answer_with_input(&mut input)
}

fn main() {
    println!("{:?}", answer());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let mut input = [0, 3, 0, 1, -3];
        assert_eq!(answer_with_input(&mut input), Answer{steps: 5});
    }
}
