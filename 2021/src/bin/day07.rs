fn parse_input(s: &str) -> impl Iterator<Item = i64> + '_ {
    s.trim().split(',').map(|x| x.parse().unwrap())
}

fn part_one(s: &str) -> String {
    let xs: Vec<_> = parse_input(s).collect();

    let cost = |m: i64| -> i64 { xs.iter().map(|&x| (x - m).abs()).sum::<i64>() };

    let min = *xs.iter().min().unwrap();
    let max = *xs.iter().max().unwrap();

    format!("{}", (min..=max).map(cost).min().unwrap())
}

fn part_two(s: &str) -> String {
    let xs: Vec<_> = parse_input(s).collect();

    let f = |x| (1..=x).sum::<i64>();
    let cost = |m: i64| { xs.iter().map(|&x| f((x-m).abs())).sum::<i64>() };

    let min = *xs.iter().min().unwrap();
    let max = *xs.iter().max().unwrap();

    format!("{}", (min..=max).map(cost).min().unwrap())
}

fn main() {
    let input = include_str!("../../input/day07.txt");
    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));
}

#[cfg(test)]
mod test {
    use super::*;
    use aocutil::test_example;

    test_example!(example_7_1, part_one, 7, 1, 1);
    test_example!(example_7_2, part_two, 7, 2, 1);
}
