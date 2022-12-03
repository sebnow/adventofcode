use anyhow::Result;
use itertools::Itertools;

type Rucksack = Vec<char>;

fn priority(c: char) -> u32 {
    if c.is_lowercase() {
        (c as u8 - b'a') as u32 + 1
    } else {
        (c as u8 - b'A') as u32 + 27
    }
}

fn appears_in_both(r: &Rucksack) -> char {
    let (a, b) = r.split_at(r.len() / 2);
    *a.iter().find(|&c| b.contains(c)).unwrap()
}

fn parse_input(s: &str) -> Result<Vec<Rucksack>> {
    Ok(s.lines().map(|l| l.chars().collect()).collect())
}

fn part_one(s: &str) -> String {
    let input = parse_input(s).unwrap();

    input
        .iter()
        .map(appears_in_both)
        .map(priority)
        .sum::<u32>()
        .to_string()
}

fn part_two(s: &str) -> String {
    let input = parse_input(s).unwrap();

    input
        .iter()
        .chunks(3)
        .into_iter()
        .map(|group| {
            let group: Vec<&Rucksack> = group.collect();
            *group[0]
                .iter()
                .find(|c| group[1..].iter().all(|o| o.contains(c)))
                .unwrap()
        })
        .map(priority)
        .sum::<u32>()
        .to_string()
}

fn main() -> Result<()> {
    let input = include_str!("../../input/day03.txt");
    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use aocutil::test_example;

    test_example!(example_3_1, part_one, 3, 1, 1);
    test_example!(example_3_2, part_two, 3, 2, 1);
}
