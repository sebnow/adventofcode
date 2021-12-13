use aocutil::Point;

type Grid = aocutil::Grid<Cell>;

struct Manual {
    grid: Grid,
    folds: Vec<Fold>,
}

#[derive(PartialEq, Copy, Clone)]
enum Fold {
    X(i64),
    Y(i64),
}

#[derive(PartialEq, Copy, Clone)]
enum Cell {
    Space,
    Dot,
}

impl Default for Cell {
    fn default() -> Self {
        Cell::Space
    }
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Cell::Dot => '#',
                Cell::Space => '.',
            }
        )
    }
}

fn fold_x(grid: &mut Grid, fold: i64) {
    let right_of_fold: Vec<Point> = grid
        .iter()
        .filter_map(|(p, _)| if p.x > fold { Some(p) } else { None })
        .copied()
        .collect();

    for p in right_of_fold {
        let c = grid.remove(&p).unwrap();
        let pp = Point::new(fold - (p.x - fold), p.y);
        grid.insert(pp, c);
    }
}

fn fold_y(grid: &mut Grid, fold: i64) {
    let above_fold: Vec<Point> = grid
        .iter()
        .filter_map(|(p, _)| if p.y > fold { Some(p) } else { None })
        .copied()
        .collect();

    for p in above_fold {
        let c = grid.remove(&p).unwrap();
        let pp = Point::new(p.x, fold - (p.y - fold));
        grid.insert(pp, c);
    }
}

fn fold<'a, I: IntoIterator<Item=&'a Fold>>(grid: &mut Grid, folds: I) {
    for &fold in folds {
        match fold {
            Fold::X(x) => fold_x(grid, x),
            Fold::Y(y) => fold_y(grid, y),
        };
    }
}

fn parse_input(s: &str) -> Manual {
    let mut parts = s.split("\n\n");

    let grid = parts
        .next()
        .unwrap()
        .lines()
        .map(|l| {
            let mut parts = l.split(',');
            let x: i64 = parts.next().unwrap().parse().expect("invalid x coordinate");
            let y: i64 = parts.next().unwrap().parse().expect("invalid y coordinate");

            (Point::new(x, y), Cell::Dot)
        })
        .collect();

    let folds = parts
        .next()
        .unwrap()
        .lines()
        .map(|l| {
            let mut parts = l.split_at(11).1.split('=');
            let axis = parts.next().unwrap().chars().next().unwrap();
            let distance = parts.next().unwrap().parse().unwrap();

            match axis {
                'y' => Fold::Y(distance),
                'x' => Fold::X(distance),
                _ => panic!("invalid axis: {}", axis),
            }
        })
        .collect();

    Manual { grid, folds }
}

fn part_one(s: &str) -> String {
    let input = parse_input(s);
    let mut grid = input.grid;

    fold(&mut grid, input.folds.iter().take(1));
    let output = grid.iter().count();

    format!("{}", output)
}

fn part_two(s: &str) -> String {
    let input = parse_input(s);
    let mut grid = input.grid;

    fold(&mut grid, &input.folds);
    // The Grid "fixes" the y-axis so the positive is rendered on top. Quick flipperooney to fix
    // that.
    fold_y(&mut grid, 0);
    format!("{}", grid).trim().to_string()
}

fn main() {
    let input = include_str!("../../input/day13.txt");
    println!("Part one: {}", part_one(input));
    println!("Part two:\n{}", part_two(input));
}

#[cfg(test)]
mod test {
    use super::*;
    use aocutil::test_example;

    test_example!(example_13_1_1, part_one, 13, 1, 1);
    test_example!(example_13_2_1, part_two, 13, 2, 1);
}
