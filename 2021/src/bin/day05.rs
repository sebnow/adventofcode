use aocutil::Point;
use std::collections::HashMap;
use std::iter::Iterator;

type Segment = (Point, Point);

fn parse_input<'a>(s: &'a str) -> impl Iterator<Item = Segment> + 'a{
    s.lines()
        .map(|l| {
            let mut points = l.split(" -> ").map(|p| {
                let mut ns = p.split(",").map(|n| n.parse().unwrap());

                Point::new(ns.next().unwrap(), ns.next().unwrap())
            });

            (points.next().unwrap(), points.next().unwrap())
        })
}

fn points((a, b): Segment) -> Vec<Point> {
    let xs: Vec<i64> = if a.x < b.x {
        (a.x..=b.x).collect()
    } else {
        ((b.x..=a.x).rev()).collect()
    };
    let ys: Vec<i64> = if a.y < b.y {
        (a.y..=b.y).collect()
    } else {
        ((b.y..=a.y).rev()).collect()
    };

    if a.x == b.x {
        xs.iter()
            .cycle()
            .zip(ys.iter())
            .map(|(&x, &y)| Point::new(x, y))
            .collect()
    } else if a.y == b.y {
        xs.iter()
            .zip(ys.iter().cycle())
            .map(|(&x, &y)| Point::new(x, y))
            .collect()
    } else {
        xs.iter()
            .zip(ys.iter())
            .map(|(&x, &y)| Point::new(x, y))
            .collect()
    }
}

fn count_crossing<I: Iterator<Item=Segment>>(segments: I) -> usize {
    let mut grid: HashMap<Point, usize> = HashMap::new();

    for s in segments {
        for p in points(s) {
            *grid.entry(p).or_insert(0) += 1;
        }
    }

    grid.values().filter(|&x| *x >= 2 as usize).count()
}

fn part_one(s: &str) -> String {
    let segments = parse_input(s);
    format!("{}", count_crossing(segments.filter(|(a, b)| a.x == b.x || a.y == b.y)))
}

fn part_two(s: &str) -> String {
    let segments = parse_input(s);
    format!("{}", count_crossing(segments))
}

fn main() {
    let input = include_str!("../../input/day05.txt");
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

#[cfg(test)]
mod test {
    use super::*;
    use aocutil::test_example;

    test_example!(example_5_1, part_one, 5, 1, 1);
    test_example!(example_5_2, part_two, 5, 2, 1);
}
