use anyhow::Result;
use aocutil;
use std::collections::HashSet;

type Point = aocutil::Point<i64>;
type Grid = aocutil::Grid<Tile>;

#[derive(PartialEq, Copy, Clone, Hash, Eq, Debug)]
pub enum Tile {
    Empty,
    Bug,
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Empty => '.',
                Tile::Bug => '#',
            }
        )
    }
}

fn count_adj_bugs(g: &Grid, p: &Point) -> usize {
    let (x, y) = (p.x, p.y);

    [
        Point::new(x - 1, y),
        Point::new(x + 1, y),
        Point::new(x, y - 1),
        Point::new(x, y + 1),
    ]
    .iter()
    .filter(|a| g.at(a).map(|&t| t == Tile::Bug).unwrap_or(false))
    .count()
}

fn step(g: &Grid) -> Grid {
    let mut new_g = Grid::new();

    for (p, t) in g.iter() {
        let adj = count_adj_bugs(g, p);

        new_g.add(
            *p,
            match (t, adj) {
                (Tile::Bug, 1) => Tile::Bug,
                (Tile::Bug, _) => Tile::Empty,
                (Tile::Empty, 1) => Tile::Bug,
                (Tile::Empty, 2) => Tile::Bug,
                (t, _) => *t,
            },
        );
    }

    new_g
}

fn find_cycle(mut grid: Grid) -> Grid {
    let mut states = HashSet::new();

    while !states.contains(&grid) {
        let g = step(&grid);
        states.insert(grid);
        grid = g;
    }

    grid
}

fn biodiversity(g: &Grid) -> usize {
    let (min_xy, max_xy) = g.bbox();

    let mut sum = 0;
    let mut points = 1;

    for y in (min_xy.y..=max_xy.y).rev() {
        for x in min_xy.x..=max_xy.x {
            let p = Point::new(x, y);
            if let Some(Tile::Bug) = g.at(&p) {
                println!("Got {} points for bug at {}", points, p);
                sum += points;
            }
            points = points << 1;
        }
    }

    sum
}

#[aoc_generator(day24)]
pub fn input_generator(input: &str) -> Grid {
    let max_y = input.lines().count() - 1;
    let tiles = input
        .lines()
        .enumerate()
        .map(|(y, l)| {
            let y = max_y - y; // Flip Y-axis
            l.trim().chars().enumerate().map(move |(x, c)| {
                let p = Point::new(x as i64, y as i64);
                let t = match c {
                    '#' => Tile::Bug,
                    '.' => Tile::Empty,
                    _ => panic!("invalid tile"),
                };
                (p, t)
            })
        })
        .flatten();

    let mut g = Grid::new();
    for (p, t) in tiles {
        g.add(p, t);
    }

    g
}

#[aoc(day24, part1)]
fn answer_1(input: &Grid) -> Result<usize> {
    let grid = find_cycle(input.to_owned());

    Ok(biodiversity(&grid))
}

#[aoc(day24, part2)]
fn answer_2(_input: &Grid) -> Result<usize> {
    Ok(0)
}

#[cfg(test)]
mod test {
    use super::*;

    fn trimmed(s: &str) -> String {
        s.lines()
            .map(|l| l.trim())
            .collect::<Vec<&str>>()
            .join("\n")
    }

    #[test]
    fn test_input() {
        assert_eq!(
            trimmed(
                r#"....#
                #..#.
                #..##
                ..#..
                #...."#
            ),
            format!(
                "{}",
                input_generator(
                    r#"....#
                    #..#.
                    #..##
                    ..#..
                    #...."#
                )
            ),
        );
    }

    #[test]
    fn example_1_steps() {
        let mut g = input_generator(
            r#"....#
            #..#.
            #..##
            ..#..
            #...."#,
        );

        g = step(&g);
        assert_eq!(
            trimmed(
                r#"#..#.
                ####.
                ###.#
                ##.##
                .##.."#
            ),
            format!("{}", g),
        );

        g = step(&g);
        assert_eq!(
            trimmed(
                r#"#####
                ....#
                ....#
                ...#.
                #.###"#
            ),
            format!("{}", g),
        );

        g = step(&g);
        assert_eq!(
            trimmed(
                r#"#....
                ####.
                ...##
                #.##.
                .##.#"#
            ),
            format!("{}", g),
        );

        g = step(&g);
        assert_eq!(
            trimmed(
                r#"####.
                ....#
                ##..#
                .....
                ##..."#
            ),
            format!("{}", g),
        );
    }

    #[test]
    fn example_1_cycle() {
        assert_eq!(
            trimmed(
                r#".....
                .....
                .....
                #....
                .#..."#
            ),
            format!(
                "{}",
                find_cycle(input_generator(
                    r#"....#
                    #..#.
                    #..##
                    ..#..
                    #...."#
                ))
            ),
        );
    }

    #[test]
    fn example_1() {
        assert_eq!(
            2129920,
            answer_1(&input_generator(
                r#"....#
                #..#.
                #..##
                ..#..
                #...."#
            ))
            .unwrap()
        );
    }
}
