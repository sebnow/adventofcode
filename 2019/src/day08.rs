use anyhow::{anyhow, Result};

const HEIGHT: usize = 6;
const WIDTH: usize = 25;

type Grid = aocutil::Grid<Color>;
type Point = aocutil::Point<i64>;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Color {
    Black,
    White,
    Transparent,
}

impl Default for Color {
    fn default() -> Self {
        Color::Transparent
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Color::White => '█',
                Color::Black => '░',
                Color::Transparent => ' ',
            }
        )
    }
}

impl std::str::FromStr for Color {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().nth(0) {
            Some('0') => Ok(Color::Black),
            Some('1') => Ok(Color::White),
            Some('2') => Ok(Color::Transparent),
            _ => Err(anyhow!("invalid color")),
        }
    }
}

pub fn decode(input: &[Color], width: usize, height: usize) -> Result<Grid> {
    let mut g = Grid::default();
    let layers: Vec<&[Color]> = input.chunks(width * height).collect();

    for y in 0..height {
        for x in 0..width {
            let i = y * width + x;
            let p = layers
                .iter()
                .find_map(|l| {
                    if l[i] != Color::Transparent {
                        Some(l[i])
                    } else {
                        None
                    }
                })
                .unwrap_or(Color::Transparent);
            g.add(Point::new(x as i64, (height - 1 - y) as i64), p);
        }
    }

    Ok(g)
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<Color> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|x| format!("{}", x).parse().expect("want color"))
        })
        .flatten()
        .collect()
}

pub fn count_value(xs: &[Color], x: Color) -> usize {
    xs.iter().filter(|&y| *y == x).count()
}

#[aoc(day8, part1)]
fn answer_1(input: &[Color]) -> Result<usize> {
    input
        .chunks(WIDTH * HEIGHT)
        .min_by(|&a, &b| count_value(a, Color::Black).cmp(&count_value(b, Color::Black)))
        .map(|l| count_value(l, Color::White) * count_value(l, Color::Transparent))
        .ok_or_else(|| anyhow!("unable to find layer"))
}

#[aoc(day8, part2)]
fn answer_2(input: &[Color]) -> Result<String> {
    Ok(format!("\n{}", decode(input, WIDTH, HEIGHT)?))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn examples_2() {
        let mut g = Grid::default();
        g.add(Point::new(0, 0), Color::White);
        g.add(Point::new(1, 0), Color::Black);
        g.add(Point::new(0, 1), Color::Black);
        g.add(Point::new(1, 1), Color::White);

        assert_eq!(
            g,
            decode(&input_generator("0222112222120000"), 2, 2).unwrap()
        );
    }
}
