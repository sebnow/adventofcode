use anyhow::Result;

fn parse_line(s: &str) -> Vec<u32> {
    s.split_once(':')
        .expect("no results")
        .1
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|n| n.trim().parse::<u32>().expect("invalid number"))
        .collect()
}

fn parse_input(s: &str) -> (Vec<u32>, Vec<u32>) {
    let parts = s.split_once('\n');
    (parse_line(parts.unwrap().0), parse_line(parts.unwrap().1))
}

fn part_one(s: &str) -> String {
    let input = parse_input(s);
    let (ts, ds) = input;

    (0..ts.len())
        .map(|idx| {
            (1..ts[idx])
                .map(|n| n * (ts[idx] - n))
                .filter(|&d| d > ds[idx])
                .count()
        })
        .product::<usize>()
        .to_string()
}

fn part_two(s: &str) -> String {
    let input = parse_input(s);
    let time = input
        .0
        .iter()
        .map(|x| x.to_string())
        .collect::<String>()
        .parse::<u64>()
        .unwrap();
    let distance = input
        .1
        .iter()
        .map(|x| x.to_string())
        .collect::<String>()
        .parse::<u64>()
        .unwrap();

    (1..time)
        .map(|n| n * (time - n))
        .filter(|&d| d > distance)
        .count()
        .to_string()
}

fn main() -> Result<()> {
    let input = include_str!("../../../../input/2023/day06.txt");
    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));

    Ok(())
}

#[cfg(test)]
mod test_day06 {
    use super::*;
    use aocutil::test_example;

    test_example!(example_1_1, part_one, 6, 1, 1);
    test_example!(example_2_1, part_two, 6, 2, 1);
}
