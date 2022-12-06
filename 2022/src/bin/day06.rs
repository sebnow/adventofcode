use anyhow::{anyhow, Result};

fn start_of_pattern(s: &[char], count_distinct: usize) -> (&[char], usize) {
    let (offset, sop) = s.windows(count_distinct).enumerate().find(|(_, window)| {
        let mut v = Vec::from(*window);
        v.sort();
        v.dedup();

        v.len() == window.len()
    }).unwrap();

    (sop, offset + count_distinct)
}

fn parse_input(s: &str) -> Result<Vec<char>> {
    Ok(s.chars().collect())
}

fn part_one(s: &str) -> String {
    let input = parse_input(s).unwrap();
    let (_, offset) = start_of_pattern(&input, 4);

    offset.to_string()
}

fn part_two(s: &str) -> String {
    let input = parse_input(s).unwrap();
    let (_, offset) = start_of_pattern(&input, 14);

    offset.to_string()
}

fn main() -> Result<()> {
    let input = include_str!("../../input/day06.txt");
    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use aocutil::test_example;

    test_example!(example_6_1_1, part_one, 6, 1, 1);
    test_example!(example_6_1_2, part_one, 6, 1, 2);
    test_example!(example_6_1_3, part_one, 6, 1, 3);
    test_example!(example_6_1_4, part_one, 6, 1, 4);
    test_example!(example_6_1_5, part_one, 6, 1, 5);
    test_example!(example_6_2_1, part_two, 6, 2, 1);
}
