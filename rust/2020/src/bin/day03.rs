use anyhow::Result;

fn hit_some_trees(s: &str, slope: (usize, usize)) -> usize {
    s.lines()
        .step_by(slope.1)
        .enumerate()
        .filter(|(d, l)| l.as_bytes()[(d * slope.0) % l.len()] == b'#')
        .count()
}

fn part_one(input: &str) -> String {
    hit_some_trees(input, (3, 1)).to_string()
}

fn part_two(input: &str) -> String {
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|&slope| hit_some_trees(input, slope))
        .product::<usize>()
        .to_string()
}

fn main() -> Result<()> {
    let input = include_str!("../../../../input/2020/day03.txt");
    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use aocutil::test_example;

    test_example!(example_one_1, part_one, 3, 1, 1);
    test_example!(example_two_1, part_two, 3, 2, 1);
}
