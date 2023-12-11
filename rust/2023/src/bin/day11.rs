use anyhow::Result;
use aocutil::Point;
use itertools::Itertools;

fn find_empties(ps: &[Point]) -> (Vec<bool>, Vec<bool>) {
    let max = ps.iter().fold(Point::new(0, 0), |acc, x| {
        Point::new(acc.x.max(x.x), acc.y.max(x.y))
    });

    let mut empty_x = vec![true; 1 + max.x as usize];
    let mut empty_y = vec![true; 1 + max.y as usize];

    for p in ps {
        empty_x[p.x as usize] = false;
        empty_y[p.y as usize] = false;
    }

    (empty_x, empty_y)
}

fn solve(input: &[Point], expand_by: u64) -> u64 {
    let empties = find_empties(input);

    input
        .iter()
        .copied()
        .tuple_combinations()
        .map(|(a, b)| {
            let empty_count = empties.0[a.x.min(b.x) as usize..a.x.max(b.x) as usize]
                .iter()
                .chain(empties.1[a.y.min(b.y) as usize..a.y.max(b.y) as usize].iter())
                .filter(|&is_empty| *is_empty)
                .count();

            a.x.abs_diff(b.x) + a.y.abs_diff(b.y) + empty_count as u64 * expand_by
        })
        .sum::<u64>()
}

fn parse_input(s: &str) -> Vec<Point> {
    s.lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .positions(|c| c == '#')
                .map(move |x| Point::new(x as i64, y as i64))
        })
        .collect()
}

fn part_one(s: &str) -> String {
    let input = parse_input(s);
    solve(&input, 1).to_string()
}

fn part_two(s: &str) -> String {
    let input = parse_input(s);
    solve(&input, 999_999).to_string()
}

fn main() -> Result<()> {
    let input = include_str!("../../../../input/2023/day11.txt");
    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));

    Ok(())
}

#[cfg(test)]
mod test_day11 {
    use super::*;
    use aocutil::test_example;

    test_example!(example_1_1, part_one, 11, 1, 1);
    test_example!(example_2_1, part_two, 11, 2, 1);
}
