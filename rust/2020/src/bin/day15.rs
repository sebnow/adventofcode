use anyhow::Result;
use std::collections::HashMap;

fn parse_input<'a>(s: &'a str) -> impl Iterator<Item = i64> + 'a {
    s.trim().split(',').map(|n| n.parse().unwrap())
}

fn play_game(input: &str, till: i64) -> i64 {
    let mut called: HashMap<i64, i64> = parse_input(input)
        .enumerate()
        .map(|(i, n)| (n, (i + 1) as i64))
        .collect();

    let mut prev = *called
        .iter()
        .max_by_key(|(_, &v)| v)
        .map(|(k, _)| k)
        .unwrap();

    for turn in called.len() as i64 + 1..=till {
        let num = match called.get(&prev) {
            Some(t) => turn - 1 - t,
            None => 0,
        };

        called.insert(prev, turn - 1);

        prev = num;
    }

    prev
}

fn part_one(input: &str) -> String {
    play_game(input, 2020).to_string()
}

fn part_two(input: &str) -> String {
    play_game(input, 30000000).to_string()
}

fn main() -> Result<()> {
    let input = include_str!("../../../../input/2020/day15.txt");
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use aocutil::test_example;

    test_example!(example_one_1, part_one, 15, 1, 1);
    //test_example!(example_two_1, part_two, 14, 2, 1);
}
