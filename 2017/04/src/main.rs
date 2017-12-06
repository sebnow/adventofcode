use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::fs::File;

#[derive(Debug, Eq, PartialEq)]
struct Answer {
    count: i32,
}

fn is_valid(passphrase: &str) -> bool {
    let mut map: HashMap<&str, ()> = HashMap::new();
    let words = passphrase.split_whitespace();

    for word in words {
        if map.contains_key(word) {
            return false
        }

        map.insert(word, ());
    }

    true
}

fn answer<R: BufRead>(reader: R) -> Answer {
    let count = reader.lines().map(|l| l.unwrap()).filter(|s| is_valid(&s)).count();

    Answer {
        count: count as i32,
    }
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    println!("{:?}", answer(reader))
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn example_answer() {
        let reader = Cursor::new("aa bb cc dd ee\naa bb cc dd aa\naa bb cc dd aaa");

        assert_eq!(answer(reader), Answer{count: 2});
    }

    #[test]
    fn example_is_valid() {
        let inputs = [
            ("aa bb cc dd ee", true),
            ("aa bb cc dd aa", false),
            ("aa bb cc dd aaa", true),
        ];

        for &(passphrase, expected) in inputs.iter() {
            assert_eq!(is_valid(passphrase), expected);
        }
    }
}
