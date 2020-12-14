use anyhow::{anyhow, Result};
use std::collections::HashMap;

enum Input {
    Mask(String),
    Assignment(usize, i64),
}

impl std::str::FromStr for Input {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(" = ");
        let a = parts.next().ok_or_else(|| anyhow!("expression missing"))?;
        let b = parts.next().ok_or_else(|| anyhow!("value missing"))?;

        Ok(match &a[0..4] {
            "mask" => Input::Mask(b.to_string()),
            _ => Input::Assignment(a[4..a.len() - 1].parse()?, b.parse()?),
        })
    }
}

fn get_value(v: i64, mask: &str) -> i64 {
    mask.chars()
        .rev()
        .enumerate()
        .fold(0, |x, (bit, m)| match m {
            'X' => x | (v & 1 << bit),
            '0' => x & !(1 << bit),
            '1' => x | 1 << bit,
            _ => panic!("wrong mask value"),
        })
}

fn get_addresses(v: usize, mask: &str) -> Vec<usize> {
    let mut addresses: Vec<usize> = vec![v];

    for (bit, m) in mask.chars().rev().enumerate() {
        let b = 1 << bit;
        match m {
            '0' => {
                for addr in &mut addresses {
                    *addr |= v & b;
                }
            }
            '1' => {
                for addr in &mut addresses {
                    *addr |= b;
                }
            }
            _ => {
                for addr in addresses.clone() {
                    addresses.push(addr ^ b);
                }
            }
        }
    }

    addresses
}

fn parse_input<'a>(s: &'a str) -> impl Iterator<Item = Input> + 'a {
    s.lines().map(|l| l.parse().unwrap())
}

fn part_one(input: &str) -> String {
    parse_input(input)
        .fold(
            (HashMap::new(), "".to_string()),
            |(mut mem, mask), input| match input {
                Input::Mask(m) => (mem, m),
                Input::Assignment(idx, v) => {
                    mem.insert(idx, get_value(v, &mask));
                    (mem, mask)
                }
            },
        )
        .0
        .values()
        .sum::<i64>()
        .to_string()
}

fn part_two(input: &str) -> String {
    let mut mem = HashMap::new();
    let mut mask = "".to_string();

    for input in parse_input(input) {
        match input {
            Input::Mask(m) => mask = m,
            Input::Assignment(idx, v) => {
                let addr = get_addresses(idx, &mask);
                for idx in addr {
                    mem.insert(idx, v);
                }
            }
        }
    }

    mem.values().sum::<i64>().to_string()
}

fn main() -> Result<()> {
    let input = include_str!("../../input/day14.txt");
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use aocutil::test_example;

    test_example!(example_one_1, part_one, 14, 1, 1);
    test_example!(example_two_1, part_two, 14, 2, 1);

    #[test]
    fn masking() {
        [
            (11, "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X", 73),
            (101, "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X", 101),
            (0, "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X", 64),
        ]
        .iter()
        .for_each(|(v, m, x)| {
            let masked = get_value(*v, m);
            assert_eq!(
                masked, *x,
                "\n{:036b}\n{}\n{:036b}\n{:036b}",
                v, m, x, masked
            );
        });
    }

    #[test]
    fn possible() {
        let mask = "00000000000000000000000000000000X0XX";
        let expected = vec![16, 17, 18, 19, 24, 25, 26, 27];
        let mut addresses = get_addresses(26, mask);
        addresses.sort();
        assert_eq!(addresses, expected);
    }
}
