use aocutil::Point;
use std::collections::HashMap;
use std::iter::Iterator;

type Segment = (Point, Point);

fn parse_input(s: &str) -> impl Iterator<Item = Segment> + '_ {
    s.lines().map(|l| {
        let mut points = l.split(" -> ").map(|p| {
            let mut ns = p.split(',').map(|n| n.parse().unwrap());

            Point::new(ns.next().unwrap(), ns.next().unwrap())
        });

        (points.next().unwrap(), points.next().unwrap())
    })
}

fn points((a, b): Segment) -> Vec<Point> {
    let mut xs: Vec<i64> = (a.x.min(b.x)..=a.x.max(b.x)).collect();
    if a.x > b.x {
        xs.reverse();
    }

    let mut ys: Vec<i64> = (a.y.min(b.y)..=a.y.max(b.y)).collect();
    if a.y > b.y {
        ys.reverse();
    }

    if xs.len() == 1 {
        xs.iter()
            .cycle()
            .zip(ys.iter())
            .map(|(&x, &y)| Point::new(x, y))
            .collect()
    } else {
        xs.iter()
            .zip(ys.iter().cycle())
            .map(|(&x, &y)| Point::new(x, y))
            .collect()
    }
}

fn count_crossing<I: Iterator<Item = Segment>>(segments: I) -> usize {
    let mut grid: HashMap<Point, usize> = HashMap::new();

    for s in segments {
        for p in points(s) {
            *grid.entry(p).or_insert(0) += 1;
        }
    }

    grid.values().filter(|&x| *x >= 2).count()
}

fn part_one(s: &str) -> String {
    let segments = parse_input(s);
    format!(
        "{}",
        count_crossing(segments.filter(|(a, b)| a.x == b.x || a.y == b.y))
    )
}

fn part_two(s: &str) -> String {
    let segments = parse_input(s);
    format!("{}", count_crossing(segments))
}

fn main() {
    let input = include_str!("../../input/day05.txt");
    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));
}

#[cfg(test)]
mod test {
    use super::*;
    use aocutil::test_example;

    test_example!(example_5_1, part_one, 5, 1, 1);
    test_example!(example_5_2, part_two, 5, 2, 1);

    #[test]
    fn diagonal() {
        assert_eq!(
            points((Point::new(0, 2), Point::new(2, 0))),
            vec![Point::new(0, 2), Point::new(1, 1), Point::new(2, 0)]
        );

        assert_eq!(
            points((Point::new(0, 0), Point::new(2, 2))),
            vec![Point::new(0, 0), Point::new(1, 1), Point::new(2, 2)]
        );
    }
}
