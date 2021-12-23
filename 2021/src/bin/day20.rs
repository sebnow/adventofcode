use aocutil::Point;

type Grid = aocutil::Grid<Cell>;

#[derive(PartialEq, Copy, Clone)]
enum Cell {
    Light,
    Dark,
}

impl Default for Cell {
    fn default() -> Self {
        Cell::Dark
    }
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            '#' => Cell::Light,
            _ => Cell::Dark,
        }
    }
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Light => '#',
            Cell::Dark => '.',
        }
        .fmt(f)
    }
}

#[inline]
fn surrounding_to_decimal(grid: &Grid, p: &Point, default: Cell) -> usize {
    IntoIterator::into_iter([
        Point::new(p.x - 1, p.y + 1),
        Point::new(p.x, p.y + 1),
        Point::new(p.x + 1, p.y + 1),
        Point::new(p.x - 1, p.y),
        Point::new(p.x, p.y),
        Point::new(p.x + 1, p.y),
        Point::new(p.x - 1, p.y - 1),
        Point::new(p.x, p.y - 1),
        Point::new(p.x + 1, p.y - 1),
    ])
    .map(move |p| (p, grid.get(&p).copied().unwrap_or(default)))
    .fold(0, |idx, (_, c)| {
        (idx << 1)
            + match c {
                Cell::Light => 1,
                Cell::Dark => 0,
            }
    })
}

fn enhance(grid: &Grid, algo: &[Cell], default: Cell) -> Grid {
    let bounds = grid.bounds();

    ((bounds.min.y - 1)..=(bounds.max.y + 1))
        .flat_map(move |y| {
            ((bounds.min.x - 1)..=(bounds.max.x + 1)).map(move |x| {
                let p = Point::new(x, y);
                let cell = algo[surrounding_to_decimal(grid, &p, default)];

                (p, cell)
            })
        })
        .collect()
}

fn parse_input(s: &str) -> (Vec<Cell>, Grid) {
    let (algo, image) = s.split_once("\n\n").unwrap();
    (
        algo.lines()
            .flat_map(|l| l.chars().map(|c| c.into()))
            .collect(),
        image.parse().unwrap(),
    )
}

fn part_one(s: &str) -> String {
    let (algo, mut grid) = parse_input(s);

    println!("{}\n", grid);
    for i in 0..2 {
        grid = enhance(&grid, &algo, if i % 2 == 0 {Cell::Dark} else {Cell::Light});
        println!("{}\n", grid);
    }

    let output = grid.iter().filter(|(_, &c)| c == Cell::Light).count();

    format!("{}", output)
}

fn part_two(s: &str) -> String {
    let input = parse_input(s);
    let output = 0;
    format!("{}", output)
}

fn main() {
    let input = include_str!("../../input/day20.txt");
    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));
}

#[cfg(test)]
mod test_day20 {
    use super::*;
    use aocutil::test_example;

    test_example!(example_20_1_1, part_one, 20, 1, 1);
    test_example!(example_20_2_1, part_two, 20, 2, 1);

    #[test]
    fn example_20_1_binary() {
        let grid: Grid = "...\n#..\n.#.".parse().unwrap();
        assert_eq!(surrounding_to_decimal(&grid, &Point::new(0, 0), Default::default()), 2);
        assert_eq!(surrounding_to_decimal(&grid, &Point::new(2, 0), Default::default()), 0);
        assert_eq!(surrounding_to_decimal(&grid, &Point::new(1, -1), Default::default()), 34);
        assert_eq!(surrounding_to_decimal(&grid, &Point::new(0, -2), Default::default()), 136);
        assert_eq!(surrounding_to_decimal(&grid, &Point::new(2, -2), Default::default()), 32);
    }
}
