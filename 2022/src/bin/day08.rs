use std::collections::VecDeque;

use anyhow::{anyhow, Context, Result};
use aocutil::{Point, MASK_CROSSHAIR};
use itertools::Itertools;

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

fn ray(height_map: &Grid, from: &Point, velocity: &Point) -> Vec<(Point, bool)> {
    let mut p = *from;
    let mut visibility = Vec::new();

    let mut prev_height: Height = height_map
        .get(&Point::new(from.x - velocity.x, from.y - velocity.y))
        .copied()
        .unwrap_or(0);

    while let Some(&h) = height_map.get(&p) {
        if h < prev_height {
            break;
        }

        visibility.push((p, true));
        prev_height = h;
        p.y += velocity.y;
        p.x += velocity.x;
    }

    visibility
}

fn in_line_of_sight(height_map: &Grid, p: &Point) -> bool {
    let h = height_map.get(p).unwrap();
    let is_shorter = |other_h| h > other_h;

    if p.x == 0
        || p.x == height_map.rows() as i64 - 1
        || p.y == 0
        || p.y == height_map.cols() as i64 - 1
    {
        return true;
    }

    println!(
        "{:?} left right {}",
        p,
        (1..p.x as i64)
            .filter_map(|x| height_map.get(&Point::new(x, p.y)))
            .all(is_shorter)
    );
    if p.x != 1
        && (1..p.x as i64)
            .filter_map(|x| height_map.get(&Point::new(x, p.y)))
            .all(is_shorter)
    {
        return true;
    }
    if p.x != height_map.cols() as i64 - 1
        && (p.x + 1..height_map.cols() as i64)
            .filter_map(|x| height_map.get(&Point::new(x, p.y)))
            .all(is_shorter)
    {
        return true;
    }

    if p.y != 1
        && (1..p.y as i64)
            .filter_map(|y| height_map.get(&Point::new(p.x, y)))
            .all(is_shorter)
    {
        return true;
    }

    if p.y != height_map.cols() as i64 - 1
        && (p.y + 1..height_map.rows() as i64)
            .filter_map(|y| height_map.get(&Point::new(p.x, y)))
            .all(is_shorter)
    {
        return true;
    }

    false
    /*
    .chain((0..=height_map.rows() as i64).map(|y| Point::new(p.x, y)))
    .filter(|other_p| p != other_p)
    .filter_map(|other_p| {
        let other_h = height_map.get(&other_p);
        println!(
            "From {:?} with height {} looking at {:?} with height {:?}",
            p, h, other_p, other_h,
        );
        other_h
    })
    .all(|other_h| h >= other_h)
    */
}

fn part_one(s: &str) -> String {
    let input = parse_input(s).unwrap();
    let height_map = input;

    height_map
        .iter()
        .filter(|(p, h)| {
            let los = in_line_of_sight(&height_map, p);
            println!("{:?} with height {:?} in line of sight? {:?}", p, h, los);
            los
        })
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
