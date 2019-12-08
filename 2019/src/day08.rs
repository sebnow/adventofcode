use anyhow::{anyhow, Result};

const HEIGHT: usize = 6;
const WIDTH: usize = 25;

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

#[derive(Debug, PartialEq)]
pub struct Image {
    width: usize,
    height: usize,
    pixels: Vec<Color>,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Self {
        let mut pixels = Vec::with_capacity(width * height);
        pixels.resize_with(pixels.capacity(), Default::default);

        Image {
            width,
            height,
            pixels,
        }
    }

    pub fn replace_linear(&mut self, i: usize, p: Color) {
        self.pixels[i] = p;
    }
}

impl std::fmt::Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, p) in self.pixels.iter().enumerate() {
            write!(f, "{}{}", if i % self.width == 0 { "\n" } else { "" }, p)?;
        }
        Ok(())
    }
}

pub fn decode(input: &[Color], width: usize, height: usize) -> Result<Image> {
    let size = width * height;
    let layers: Vec<&[Color]> = input.chunks(size).collect();
    let mut image = Image::new(width, height);

    for i in 0..size {
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

        image.replace_linear(i, p);
    }

    Ok(image)
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
fn answer_2(input: &[Color]) -> Result<Image> {
    decode(input, WIDTH, HEIGHT)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn examples_2() {
        let mut image = Image::new(2, 2);
        image.replace_linear(0, Color::Black);
        image.replace_linear(1, Color::White);
        image.replace_linear(2, Color::White);
        image.replace_linear(3, Color::Black);

        assert_eq!(
            image,
            decode(&input_generator("0222112222120000"), 2, 2).unwrap()
        );
    }
}
