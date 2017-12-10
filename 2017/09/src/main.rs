use std::fs::File;
use std::io::Read;

fn score(input: &str) -> (i32, i32) {
    let mut scores = vec![];
    let mut depth = 1;
    let mut is_cancelled = false;
    let mut is_garbage = false;
    let mut garbage = 0;

    for c in input.chars() {
        if is_cancelled {
            is_cancelled = false;
            continue;
        }

        match c {
            '{' => {
                if is_garbage {
                    garbage += 1;
                } else {
                    scores.push(depth);
                    depth += 1;
                }
            },
            '}' => {
                if is_garbage {
                    garbage += 1;
                } else {
                    depth -= 1;
                }
            },
            '<' => {
                if is_garbage {
                    garbage += 1;
                }
                is_garbage = true;
            },
            '>' => {
                is_garbage = false;
            },
            '!' => {
                is_cancelled = !is_cancelled;
            }
            _ => {
                if is_garbage {
                    garbage += 1;
                }
            }
        }
    }

    (scores.iter().sum(), garbage)
}

fn main() {
    let mut input = String::new();
    let mut file = File::open("input.txt").unwrap();
    file.read_to_string(&mut input).unwrap();

    let answer = score(&input);
    println!("Part 1: {:?}", answer.0);
    println!("Part 2: {:?}", answer.1);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn examples_1() {
        assert_eq!(score("{}").0, 1);
        assert_eq!(score("{{{}}}").0, 6);
        assert_eq!(score("{{},{}}").0, 5);
        assert_eq!(score("{{{},{},{{}}}}").0, 16);
        assert_eq!(score("{<a>,<a>,<a>,<a>}").0, 1);
        assert_eq!(score("{{<ab>},{<ab>},{<ab>},{<ab>}}").0, 9);
        assert_eq!(score("{{<!!>},{<!!>},{<!!>},{<!!>}}").0, 9);
        assert_eq!(score("{{<a!>},{<a!>},{<a!>},{<ab>}}").0, 3);
    }

    #[test]
    fn examples_2() {
        assert_eq!(score("<>").1, 0);
        assert_eq!(score("<random characters>").1, 17);
        assert_eq!(score("<<<<>").1, 3);
        assert_eq!(score("<{!>}>").1, 2);
        assert_eq!(score("<!!>").1, 0);
        assert_eq!(score("<!!!>>").1, 0);
        assert_eq!(score("<{o\"i!a,<{i<a>").1, 10);
    }
}
