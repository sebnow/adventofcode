use aocutil::{Point, Vector};
use std::ops::Neg;

type Box = euclid::Box2D<i64, euclid::UnknownUnit>;

#[derive(PartialEq, Clone, Debug)]
struct Probe {
    velocity: Vector,
    position: Point,
}

impl Probe {
    fn step(&self) -> Self {
        let drag = 0.cmp(&self.velocity.x) as i64;
        let gravity = -1;
        let dravity = Vector::new(drag, gravity);

        Probe {
            velocity: self.velocity + dravity,
            position: self.position + self.velocity,
        }
    }
}

fn parse_input(s: &str) -> Box {
    let (x, y) = &s.trim()[13..].split_once(", ").unwrap();
    let parse_bounds = |s: &str| {
        s[2..]
            .split_once("..")
            .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
            .unwrap()
    };
    let x_bounds = parse_bounds(x);
    let y_bounds = parse_bounds(y);

    Box::new(
        Point::new(x_bounds.0, y_bounds.0),
        Point::new(x_bounds.1, y_bounds.1),
    )
}

fn get_trajectory(velocity: &Vector, target: &Box) -> Option<Vec<Probe>> {
    let mut probe = Probe {
        position: Point::new(0, 0),
        velocity: *velocity,
    };
    let mut path: Vec<Probe> = vec![];

    loop {
        probe = probe.step();
        path.push(probe.clone());

        if target.min.x <= probe.position.x && probe.position.x <= target.max.x && target.min.y <= probe.position.y && probe.position.y <= target.max.y {
            return Some(path);
        }

        if probe.position.x < target.min.x && probe.velocity.x < 1 {
            return None
        }
        if probe.position.x > target.max.x || probe.position.y < target.min.y {
            return None;
        }
    }
}

fn find_velocity(target: &Box) -> Vec<(Vector, Vec<Probe>)> {
    let mut vs = vec![];

    let search_space: i64 = 500;
    for x in 1..search_space {
        for y in search_space.neg()..search_space {
            let v = Vector::new(x, y);
            match get_trajectory(&v, target) {
                Some(p) => vs.push((v, p)),
                None => continue,
            }
        }
    }

    vs
}

fn part_one(s: &str) -> String {
    let input = parse_input(s);
    let possible = find_velocity(&input);

    let output = possible
        .iter()
        .flat_map(|(_, path)| path.iter().map(|probe| probe.position.y))
        .max()
        .unwrap();

    format!("{}", output)
}

fn part_two(s: &str) -> String {
    let input = parse_input(s);
    let possible = find_velocity(&input);

    let output = possible.len();

    format!("{}", output)
}

fn main() {
    let input = include_str!("../../../../input/2021/day17.txt");
    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));
}

#[cfg(test)]
mod test_day17 {
    use super::*;
    use aocutil::test_example;

    test_example!(example_17_1_1, part_one, 17, 1, 1);
    test_example!(example_17_2_1, part_two, 17, 2, 1);
}
