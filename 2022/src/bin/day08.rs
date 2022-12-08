use anyhow::{Context, Result};
use aocutil::Point;

type Height = u32;
type Grid = aocutil::Grid<Height>;

fn parse_input(s: &str) -> Result<Grid> {
    s.lines()
        .rev()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars().enumerate().map(move |(x, c)| {
                Ok((
                    Point::new(x as i64, y as i64),
                    c.to_digit(10)
                        .with_context(|| "failed to parse tree height")?
                        as Height,
                ))
            })
        })
        .collect()
}

fn in_line_of_sight(height_map: &Grid, p: &Point) -> bool {
    let h = height_map.get(p).unwrap();
    let is_lower = |other_h| other_h < h;

    (0..p.x)
        .filter_map(|x| height_map.get(&Point::new(x, p.y)))
        .all(is_lower)
        || (p.x + 1..height_map.rows() as i64)
            .filter_map(|x| height_map.get(&Point::new(x, p.y)))
            .all(is_lower)
        || (0..p.y)
            .filter_map(|y| height_map.get(&Point::new(p.x, y)))
            .all(is_lower)
        || (p.y + 1..height_map.cols() as i64)
            .filter_map(|y| height_map.get(&Point::new(p.x, y)))
            .all(is_lower)
}

fn part_one(s: &str) -> String {
    let input = parse_input(s).unwrap();
    let height_map = input;

    height_map
        .iter()
        .filter(|(p, _)| in_line_of_sight(&height_map, p))
        .count()
        .to_string()
}

fn part_two(s: &str) -> String {
    let input = parse_input(s).unwrap();
    "".to_string()
}

fn main() -> Result<()> {
    let input = include_str!("../../input/day08.txt");
    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use aocutil::test_example;

    test_example!(example_8_1, part_one, 8, 1, 1);
    //test_example!(example_8_2, part_two, 8, 2, 1);
}
