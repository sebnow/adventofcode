use anyhow::Result;
use aocutil::Point;

type Grid = aocutil::Grid<Cell>;
type Cell = char;

fn next_point(p: &Point, d: Dir) -> Point {
    match d {
        Dir::North => Point::new(p.x, p.y + 1),
        Dir::South => Point::new(p.x, p.y - 1),
        Dir::East => Point::new(p.x + 1, p.y),
        Dir::West => Point::new(p.x - 1, p.y),
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Dir {
    North,
    East,
    South,
    West,
}

impl Dir {
    fn inverse(&self) -> Dir {
        use Dir::*;
        match self {
            North => South,
            East => West,
            South => North,
            West => East,
        }
    }
}

fn directions(p: Cell) -> Option<[Dir; 2]> {
    match p {
        'J' => Some([Dir::North, Dir::West]),
        'L' => Some([Dir::North, Dir::East]),
        '|' => Some([Dir::North, Dir::South]),
        '-' => Some([Dir::West, Dir::East]),
        'F' => Some([Dir::South, Dir::East]),
        '7' => Some([Dir::South, Dir::West]),
        _ => None,
    }
}

fn get_point(grid: &Grid, p: &Point) -> Cell {
    *grid.get(p).unwrap_or(&'.')
}

fn find_loop(grid: &Grid) -> Vec<Point> {
    let mut pos = *grid.iter().find(|(_, &c)| c == 'S').unwrap().0;
    let mut dir = Dir::North;
    let mut path = Vec::new();

    if ['|', '7', 'F'].contains(&get_point(grid, &next_point(&pos, Dir::North))) {
        dir = Dir::North;
        pos.y += 1;
    }
    if ['-', '7', 'J'].contains(&get_point(grid, &next_point(&pos, Dir::East))) {
        dir = Dir::East;
        pos.x += 1;
    }
    if ['|', 'L', 'J'].contains(&get_point(grid, &next_point(&pos, Dir::South))) {
        dir = Dir::South;
        pos.y -= 1;
    }

    loop {
        path.push(pos);
        let c = *grid.get(&pos).unwrap();
        if c == 'S' {
            break;
        }

        let next_dirs = directions(c).unwrap();
        let next_dir = if next_dirs[0] == dir.inverse() {
            next_dirs[1]
        } else {
            next_dirs[0]
        };

        pos = next_point(&pos, next_dir);
        dir = next_dir;
    }

    path
}

fn is_point_inside_polygon(p: &Point, polygon: &[Point]) -> bool {
    polygon
        .iter()
        .zip(polygon.iter().cycle().skip(1))
        .fold(false, |inside, (p1, p2)| {
            let intersect = ((p1.y > p.y) != (p2.y > p.y))
                && (p.x < (p2.x - p1.x) * (p.y - p1.y) / (p2.y - p1.y) + p1.x);

            if intersect {
                !inside
            } else {
                inside
            }
        })
}

fn parse_input(s: &str) -> Grid {
    s.lines()
        .rev()
        .enumerate()
        .flat_map(move |(y, l)| {
            l.chars()
                .enumerate()
                .map(move |(x, c)| (Point::new(x as i64, y as i64), c))
        })
        .collect()
}

fn part_one(s: &str) -> String {
    let input = parse_input(s);

    (find_loop(&input).len() / 2).to_string()
}

fn part_two(s: &str) -> String {
    let input = parse_input(s);
    let polygon = find_loop(&input);

    input
        .iter()
        .filter(|(p, _)| !polygon.contains(p))
        .filter(|(p, _)| is_point_inside_polygon(p, &polygon))
        .count()
        .to_string()
}

fn main() -> Result<()> {
    let input = include_str!("../../../../input/2023/day10.txt");
    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));

    Ok(())
}

#[cfg(test)]
mod test_day10 {
    use super::*;
    use aocutil::test_example;

    test_example!(example_1_1, part_one, 10, 1, 1);
    test_example!(example_1_2, part_one, 10, 1, 2);
    test_example!(example_2_1, part_two, 10, 2, 1);
}
