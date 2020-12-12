use anyhow::{anyhow, Result};

fn parse_input(s: &str) -> Result<Vec<i64>> {
    s.lines()
        .map(|l| l.parse().map_err(|err| anyhow!("failed to parse; {}", err)))
        .collect()
}

fn part_one(input: &str) -> String {
    let mut adapters = parse_input(input).unwrap();
    adapters.insert(0, 0);
    adapters.sort();
    adapters.push(adapters.last().unwrap() + 3);

    let res = adapters
        .iter()
        .zip(adapters.iter().skip(1))
        .fold((0, 0), |(one, three), (a, b)| {
            let diff = b - a;
            if diff == 3 {
                return (one, three + 1);
            } else if diff == 1 {
                return (one + 1, three);
            }
            if diff > 3 {
                panic!("{} - {} > 3", b, a);
            }
            (one, three)
        });

    (res.0 * res.1).to_string()
}

fn part_two(input: &str) -> String {
    let perms = [1, 1, 1, 2, 4, 7];
    let mut adapters = parse_input(input).unwrap();
    adapters.insert(0, 0);
    adapters.sort();
    adapters.push(adapters.last().unwrap() + 3);

    adapters
        .iter()
        .zip(adapters.iter().skip(1))
        .fold((1, 0), |(x, s): (i64, i64), (a, b)| {
            if b - a == 3 {
                (x * perms[s as usize + 1], 0)
            } else {
                (x, s + 1)
            }
        })
        .0
        .to_string()
}

fn main() -> Result<()> {
    let input = include_str!("../../input/day10.txt");
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use aocutil::test_example;

    test_example!(example_one_1, part_one, 10, 1, 1);
    test_example!(example_one_2, part_one, 10, 1, 2);
    test_example!(example_two_1, part_two, 10, 2, 1);
    test_example!(example_two_2, part_two, 10, 2, 2);
}
