use anyhow::Result;
use aocutil::Point;

type PartId = u64;
type EntityId = usize;

#[derive(Copy, Clone, PartialEq, Default)]
enum Cell {
    Number(u32),
    Symbol(char),
    #[default]
    Empty,
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Number(n) => write!(f, "{}", n),
            Cell::Symbol(c) => write!(f, "{}", c),
            Cell::Empty => write!(f, "."),
        }
    }
}

type Grid = aocutil::Grid<Cell>;

fn parse_input(s: &str) -> Grid {
    s.lines()
        .rev()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars().enumerate().map(move |(x, c)| {
                (
                    Point::new(x as i64, y as i64),
                    match c {
                        '0'..='9' => Cell::Number(c.to_digit(10).expect("invalid digit")),
                        '.' => Cell::Empty,
                        _ => Cell::Symbol(c),
                    },
                )
            })
        })
        .collect()
}

fn get_entities(grid: &Grid) -> (aocutil::Grid<EntityId>, Vec<PartId>) {
    let mut entities = Vec::new();
    let mut entity_map = aocutil::Grid::new();

    let mut in_part_id;
    for y in 0..=grid.rows() {
        in_part_id = false;
        for x in 0..=grid.cols() {
            let p = Point::new(x as i64, y as i64);
            match (in_part_id, grid.get(&p)) {
                // New part ID found
                (false, Some(&Cell::Number(n))) => {
                    in_part_id = true;
                    entities.push(n as PartId);
                    entity_map.insert(p, entities.len() - 1);
                }
                // Same part ID
                (true, Some(&Cell::Number(b))) => {
                    let entity_id = entities.len() - 1;
                    entities[entity_id] = entities[entity_id] * 10 + b as PartId;
                    entity_map.insert(p, entity_id);
                }
                // Part ID ended
                (true, None) | (true, Some(Cell::Empty)) | (true, Some(Cell::Symbol(_))) => {
                    in_part_id = false;
                }
                // Ignore
                (false, _) => {}
            };
        }
    }

    (entity_map, entities)
}

fn part_one(s: &str) -> String {
    let input = parse_input(s);
    let (entity_map, entities) = get_entities(&input);

    let mut adj: Vec<EntityId> =
        input.iter().fold(Vec::new(), |mut acc, (point, cell)| {
            if matches!(cell, Cell::Symbol(_)) {
                acc.extend(input.surrounding(point, aocutil::MASK_ALL).filter_map(
                    |(p, c)| match c {
                        Cell::Number(_) => entity_map.get(&p),
                        _ => None,
                    },
                ));
            }
            acc
        });

    adj.sort();
    adj.dedup();
    adj.iter()
        .map(|&id| entities[id])
        .sum::<PartId>()
        .to_string()
}

fn part_two(s: &str) -> String {
    let input = parse_input(s);
    let (entity_map, entities) = get_entities(&input);

    let gears: Vec<Vec<&EntityId>> = input.iter().fold(Vec::new(), |mut acc, (point, cell)| {
        if matches!(cell, Cell::Symbol('*')) {
            let mut entities = input
                .surrounding(point, aocutil::MASK_ALL)
                .filter_map(|(p, c)| match c {
                    Cell::Number(_) => entity_map.get(&p),
                    _ => None,
                })
                .collect::<Vec<_>>();
            entities.sort();
            entities.dedup();
            if entities.len() == 2 {
                acc.push(entities);
            }
        }
        acc
    });

    gears
        .into_iter()
        .map(|gears| {
            gears
                .into_iter()
                .map(|&id| entities[id])
                .product::<PartId>()
        })
        .sum::<PartId>()
        .to_string()
}

fn main() -> Result<()> {
    let input = include_str!("../../../../input/2023/day03.txt");
    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));

    Ok(())
}

#[cfg(test)]
mod test_day03 {
    use super::*;
    use aocutil::test_example;

    test_example!(example_1_1, part_one, 3, 1, 1);
    test_example!(example_2_1, part_two, 3, 2, 1);
}
