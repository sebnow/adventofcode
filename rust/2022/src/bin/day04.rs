use anyhow::Result;
use itertools::Itertools;

type Range = (u32, u32);
type Pair = [Range; 2];

fn parse_range(s: &str) -> Result<Range> {
    let mut parts = s.split('-');
    Ok((
        parts.next().unwrap().parse().unwrap(),
        parts.next().unwrap().parse().unwrap(),
    ))
}

fn parse_input(s: &str) -> Result<Vec<Pair>> {
    Ok(s.lines()
        .map(|l| {
            let mut parts = l.split(',');
            [
                parse_range(parts.next().unwrap()).unwrap(),
                parse_range(parts.next().unwrap()).unwrap(),
            ]
        })
        .collect())
}

fn intersects(r1: &Range, r2: &Range) -> bool {
    (r2.0 <= r1.0 && r1.0 <= r2.1) || (r2.0 <= r1.1 && r1.1 <= r2.1)
}

fn pair_intersects(pair: &Pair) -> bool {
    intersects(&pair[0], &pair[1]) || intersects(&pair[1], &pair[0])
}

fn overlaps(r1: &Range, r2: &Range) -> bool {
    r1.0 <= r2.0 && r1.1 >= r2.1
}

fn pair_fully_overlaps(pair: &Pair) -> bool {
    overlaps(&pair[0], &pair[1]) || overlaps(&pair[1], &pair[0])
}

fn part_one(s: &str) -> String {
    let input = parse_input(s).unwrap();

    input
        .into_iter()
        .filter(pair_fully_overlaps)
        .count()
        .to_string()
}

fn part_two(s: &str) -> String {
    let input = parse_input(s).unwrap();

    input
        .into_iter()
        .filter(pair_intersects)
        .count()
        .to_string()
}

fn main() -> Result<()> {
    let input = include_str!("../../../../input/2022/day04.txt");
    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use aocutil::test_example;

    test_example!(example_4_1, part_one, 4, 1, 1);
    test_example!(example_4_2, part_two, 4, 2, 1);
}
