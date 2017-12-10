use std::fs::File;
use std::io::Read;

fn score(input: &str) -> i32 {
    let mut scores = vec![];
    let mut depth = 1;
    let mut is_cancelled = false;
    let mut is_garbage = false;

    for c in input.chars() {
        if is_cancelled {
            is_cancelled = false;
            continue;
        }

        match c {
            '{' => {
                if !is_garbage {
                    scores.push(depth);
                    depth += 1;
                }
            },
            '}' => {
                if !is_garbage {
                    depth -= 1;
                }
            },
            '<' => {
                is_garbage = true;
            },
            '>' => {
                is_garbage = false;
            },
            '!' => {
                is_cancelled = !is_cancelled;
            }
            _ => {}
        }
    }

    scores.iter().sum()
}

fn answer_1() -> i32 {
    let mut input = String::new();
    let mut file = File::open("input.txt").unwrap();
    file.read_to_string(&mut input).unwrap();

    score(&input)
}

fn main() {
    println!("Part 1: {:?}", answer_1());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn examples_1() {
        assert_eq!(score("{}"), 1);
        assert_eq!(score("{{{}}}"), 6);
        assert_eq!(score("{{},{}}"), 5);
        assert_eq!(score("{{{},{},{{}}}}"), 16);
        assert_eq!(score("{<a>,<a>,<a>,<a>}"), 1);
        assert_eq!(score("{{<ab>},{<ab>},{<ab>},{<ab>}}"), 9);
        assert_eq!(score("{{<!!>},{<!!>},{<!!>},{<!!>}}"), 9);
        assert_eq!(score("{{<a!>},{<a!>},{<a!>},{<ab>}}"), 3);
    }
}
