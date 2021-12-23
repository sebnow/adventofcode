use anyhow::Result;
use aocutil::Point;
use rand::Rng;

type Grid = aocutil::Grid<Cell>;

#[derive(PartialEq, Copy, Clone)]
enum Cell {
    Empty,
    Space,
    Wall,
    Amphipod(Amphipod),
}

impl aocutil::Pathfindable for Cell {
    fn collides_with(&self, other: &Self) -> bool {
        match (self, other) {
            (Cell::Amphipod(_), Cell::Wall) => true,
            (Cell::Amphipod(_), Cell::Amphipod(_)) => true,
            _ => false,
            }
    }

    fn traverse_cost(&self) -> i64 {
        1
    }

    fn direction_mask(&self) -> u8 {
        aocutil::MASK_CROSSHAIR
    }
}

impl Default for Cell {
    fn default() -> Self {
        Cell::Empty
    }
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            '#' => Cell::Wall,
            ' ' => Cell::Empty,
            '.' => Cell::Space,
            'A'..='D' => Cell::Amphipod(Amphipod::new(c)),
            _ => panic!("invalid cell {}", c),
        }
    }
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Wall => '#',
            Cell::Empty => ' ',
            Cell::Space => '.',
            Cell::Amphipod(c) => c.kind,
        }
        .fmt(f)
    }
}

#[derive(PartialEq, Copy, Clone, Default)]
struct Amphipod {
    id: u64,
    kind: char,
    energy_used: u64,
}

impl Amphipod {
    pub fn new(kind: char) -> Self {
        let mut rng = rand::thread_rng();
        Amphipod {
            id: rng.gen(),
            kind,
            ..Default::default()
        }
    }

    pub fn move_to(&self, grid: &mut Grid, p: &Point) -> Result<()> {}

    fn energy(&self) -> u64 {
        match self.kind {
            'A' => 1,
            'B' => 10,
            'C' => 100,
            'D' => 1_000,
            _ => 0,
        }
    }
}

fn parse_input(s: &str) -> Grid {
    s.parse().unwrap()
}

fn part_one(s: &str) -> String {
    let input = parse_input(s);
    println!("{}", input);

    let output = 0;

    format!("{}", output)
}

fn part_two(s: &str) -> String {
    let input = parse_input(s);

    let output = 0;

    format!("{}", output)
}

fn main() {
    let input = include_str!("../../input/day23.txt");
    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));
}

#[cfg(test)]
mod test_day23 {
    use super::*;
    use aocutil::test_example;

    test_example!(example_23_1_1, part_one, 23, 1, 1);
    test_example!(example_23_2_1, part_two, 23, 2, 1);
}
