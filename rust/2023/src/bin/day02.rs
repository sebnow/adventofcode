use anyhow::Result;

#[derive(PartialEq)]
enum Cube {
    Red,
    Green,
    Blue,
}

type Subset = Vec<(u32, Cube)>;

struct Game {
    id: u32,
    subsets: Vec<Subset>,
}

fn parse_input(s: &str) -> Vec<Game> {
    s.lines()
        .map(|l| {
            let (left, right) = l.split_once(": ").expect("missing :");
            let id = left
                .split_once(' ')
                .expect("missing game ID")
                .1
                .parse()
                .expect("invalid id");

            let subsets = right
                .split("; ")
                .map(|s| {
                    s.split(", ")
                        .map(|cs| {
                            let (count, color) = cs.split_once(' ').expect("invalid cube");
                            (
                                count.parse().expect("invalid count"),
                                match color {
                                    "red" => Cube::Red,
                                    "green" => Cube::Green,
                                    "blue" => Cube::Blue,
                                    _ => panic!("invalid color"),
                                },
                            )
                        })
                        .collect()
                })
                .collect();
            Game { id, subsets }
        })
        .collect()
}

fn find_max_by_color(g: &Game, needle: Cube) -> u32 {
    g.subsets
        .iter()
        .flat_map(|s| {
            s.iter()
                .filter_map(|c| if c.1 == needle { Some(c.0) } else { None })
        })
        .max_by(|a, b| a.cmp(b))
        .unwrap_or(0)
}

fn find_min_by_color(g: &Game, needle: Cube) -> u32 {
    g.subsets
        .iter()
        .flat_map(|s| {
            s.iter()
                .filter_map(|c| if c.1 == needle { Some(c.0) } else { None })
        })
        .max_by(|a, b| a.cmp(b))
        .unwrap_or(0)
}

fn part_one(s: &str) -> String {
    let input = parse_input(s);
    input
        .iter()
        .filter_map(|g| {
            let reds = find_max_by_color(g, Cube::Red);
            let greens = find_max_by_color(g, Cube::Green);
            let blues = find_max_by_color(g, Cube::Blue);
            if reds <= 12 && greens <= 13 && blues <= 14 {
                Some(g.id)
            } else {
                None
            }
        })
        .sum::<u32>()
        .to_string()
}

fn part_two(s: &str) -> String {
    let _input = parse_input(s);
    let input = parse_input(s);
    input
        .iter()
        .map(|g| {
            let reds = find_min_by_color(g, Cube::Red);
            let greens = find_min_by_color(g, Cube::Green);
            let blues = find_min_by_color(g, Cube::Blue);
            reds * greens * blues
        })
        .sum::<u32>()
        .to_string()
}

fn main() -> Result<()> {
    let input = include_str!("../../../../input/2023/day02.txt");
    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));

    Ok(())
}

#[cfg(test)]
mod test_day02 {
    use super::*;
    use aocutil::test_example;

    test_example!(example_1_1, part_one, 2, 1, 1);
    test_example!(example_2_1, part_two, 2, 2, 1);
}
