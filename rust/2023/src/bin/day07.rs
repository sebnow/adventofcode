use anyhow::Result;
use itertools::Itertools;

type Values = [char; 13];
const STRENGTH_P1: Values = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];
const STRENGTH_P2: Values = [
    'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
];

type Hand<'a> = &'a str;

fn parse_input(s: &str) -> impl Iterator<Item = (Hand, u64)> {
    s.lines().map(|l| {
        let (h, b) = l.split_once(' ').unwrap();
        (h, b.parse().unwrap())
    })
}

fn solve<'a, I: Iterator<Item = (Hand<'a>, u64)>>(input: I, values: Values) -> u64 {
    input
        .map(|(h, bid)| {
            let mut counts: [usize; 13] = [0; 13];
            for c in h.chars() {
                let p = values.iter().position(|&x| x == c).unwrap();
                counts[p] += 1;
            }

            // For part 1: No jokers will be removed, and thus none will be added.
            // For part 2: Remove the jokers to add to the highest value card.
            if values[0] == 'J' && counts[0] != 5 {
                counts[0] = 0;
            }

            let mut strength = counts
                .into_iter()
                .filter(|&x| x > 0)
                .sorted()
                .rev()
                .collect_vec();
            strength[0] += h.len() - strength.iter().sum::<usize>();
            strength.extend(
                h.chars()
                    .map(|c| values.iter().position(|&x| x == c).unwrap()),
            );

            (h, bid, strength)
        })
        .sorted_by_key(|(_, _, s)| s.clone())
        .enumerate()
        .map(|(rank, (_, bid, _))| (rank + 1) as u64 * bid)
        .sum::<u64>()
}

fn part_one(s: &str) -> String {
    let input = parse_input(s);
    solve(input, STRENGTH_P1).to_string()
}

fn part_two(s: &str) -> String {
    let input = parse_input(s);
    solve(input, STRENGTH_P2).to_string()
}

fn main() -> Result<()> {
    let input = include_str!("../../../../input/2023/day07.txt");
    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));

    Ok(())
}

#[cfg(test)]
mod test_day07 {
    use super::*;
    use aocutil::test_example;

    test_example!(example_1_1, part_one, 7, 1, 1);
    test_example!(example_2_1, part_two, 7, 2, 1);
}
