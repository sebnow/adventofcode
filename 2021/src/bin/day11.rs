use aocutil::{Point, MASK_ALL};
use std::collections::{HashSet, VecDeque};

type Grid = aocutil::Grid<OctopusEnergy>;
type OctopusEnergy = u8;

const FLASH_ENERGY: OctopusEnergy = 10;

fn parse_input(s: &str) -> Grid {
    s.lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars().enumerate().map(move |(x, c)| {
                (
                    Point::new(x as i64, 0 - y as i64),
                    c.to_digit(10).expect("digit") as OctopusEnergy,
                )
            })
        })
        .collect()
}

fn step(old: &Grid) -> (Grid, usize) {
    let mut queue: VecDeque<Point> = VecDeque::new();
    let mut flashed: HashSet<Point> = HashSet::new();

    // Increase by one
    let mut grid: Grid = old.iter().map(|(&p, o)| (p, o + 1)).collect();

    // Increase those surrounding 10
    loop {
        queue.extend(
            grid.iter()
                .filter(|(p, &o)| o >= FLASH_ENERGY && !flashed.contains(p))
                .map(|x| x.0),
        );

        if queue.is_empty() {
            break;
        }

        // Flash
        while let Some(p) = queue.pop_front() {
            flashed.insert(p);
            let surr: Vec<_> = grid.surrounding(&p, MASK_ALL).map(|x| x.0).collect();
            for s in surr {
                let o = grid.get_mut(&s).unwrap();
                *o += 1;
            }
        }
    }

    // Reset all with 10 to 0
    for p in &flashed {
        let o = grid.get_mut(p).unwrap();
        *o = 0;
    }

    (grid, flashed.len())
}

fn part_one(s: &str) -> String {
    let grid = parse_input(s);

    let (_, output) = (1..=100).fold((grid, 0), |(grid, flashes), _| {
        let (g, new) = step(&grid);
        (g, flashes + new)
    });
    format!("{}", output)
}

fn part_two(s: &str) -> String {
    let mut grid = parse_input(s);
    let mut output = 0;

    loop {
        let (new_grid, flashes) = step(&grid);
        output += 1;

        grid = new_grid;
        if flashes == grid.len() {
            break;
        }
    }

    format!("{}", output)
}

fn main() {
    let input = include_str!("../../input/day11.txt");
    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));
}

#[cfg(test)]
mod test {
    use super::*;
    use aocutil::test_example;

    test_example!(example_11_1, part_one, 11, 1, 1);
    test_example!(example_11_2, part_two, 11, 2, 1);
}
