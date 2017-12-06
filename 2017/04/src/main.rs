fn is_valid(passphrase: &str) -> bool {
    false
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::*;

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
