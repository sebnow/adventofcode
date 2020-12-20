use aocutil::{Grid, Point};
use std::collections::{HashMap, VecDeque};

type Vector = euclid::Vector2D<i64, euclid::UnknownUnit>;

#[derive(Clone, Debug, PartialEq)]
struct Image {
    id: i64,
    grid: Grid<char>,
}

impl Image {
    fn bordering(&self, other: &Self) -> Option<(Vector, Self)> {
        todo!()
    }
}

fn stitch(images: &[Image]) -> Grid<char> {
    let mut queue: VecDeque<&Image> = images.iter().collect();
    let mut stitched: Grid<&Image> = {
        let first = queue.pop_front().unwrap();
        let mut g = Grid::new();
        g.insert(Point::zero(), first);
        g
    };

    'queue: while let Some(img) = queue.pop_front() {
        let old = stitched.clone();
        for (&p, c) in old.iter() {
            if let Some((dp, transformed)) = c.bordering(&img) {
                stitched.insert(p + dp, &transformed);
                continue 'queue;
            }
        }
        queue.push_back(img);
    }

    // let complete = Grid::new();
    // //.. Copy images, discarding borders
    Grid::new()
}

fn parse_input<'a>(input: &'a str) -> Vec<Image> {
    input
        .split("\n\n")
        .map(|tile| {
            let mut lines = tile.lines();
            let id = lines
                .next()
                .expect("missing tile header")
                .split(" ")
                .nth(1)
                .map(|id| id[0..id.len() - 1].parse().expect("invalid id"))
                .expect("missing id");
            let mut grid = Grid::new();

            for (dy, l) in lines.enumerate() {
                for (x, c) in l.chars().enumerate() {
                    grid.insert(Point::new(x as i64, 0 - dy as i64), c);
                }
            }

            Image { id, grid }
        })
        .collect()
}

fn part_one(input: &str) -> String {
    let images = parse_input(input);
    let mut borders: HashMap<String, Vec<&Image>> = HashMap::new();

    for img in &images {
        let g = &img.grid;
        let min_y = 0 - (g.rows() as i64);
        let max_x = g.cols() as i64;

        let get_cell = |x, y| {
            g.get(&Point::new(x, y))
                .expect(&format!("cell missing at {},{}", x, y))
        };

        let left: String = (min_y..=0).map(|y| get_cell(0, y)).collect();
        let right: String = (min_y..=0).map(|y| get_cell(max_x, y)).collect();
        let top: String = (0..=max_x).map(|x| get_cell(x, 0)).collect();
        let bottom: String = (0..=max_x).map(|x| get_cell(x, min_y)).collect();

        for border in &[left, top, right, bottom] {
            borders
                .entry(border.to_owned())
                .or_insert(Vec::new())
                .push(img);
            let rev = border.chars().rev().collect();
            borders.entry(rev).or_insert(Vec::new()).push(&img);
        }
    }

    let mut corner = HashMap::new();
    // Borders on the edges will not touch any other image. For each border matching only one
    // image, add it to a HashMap<ID, Vec<Border>>. This lets us find the edge images which have
    // two borders, i.e. corners.
    borders
        .iter()
        .filter(|(_, imgs)| imgs.len() == 1)
        .flat_map(|(border, imgs)| imgs.iter().map(move |i| (border, i)))
        .for_each(|(border, img)| corner.entry(img.id).or_insert(Vec::new()).push(border));

    // Corners will have two edge borders. Images in the middle will only have one. Since both
    // permutations of the border are added, the amount of images is doubled.
    corner
        .iter()
        .filter(|(_, imgs)| imgs.len() == 4)
        .map(|(id, _)| id)
        .product::<i64>()
        .to_string()
}

fn part_two(input: &str) -> String {
    let g = stitch(&parse_input(input));

    "".to_string()
}

fn main() {
    let input = include_str!("../../input/day20.txt");
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

#[cfg(test)]
mod test {
    use super::*;
    use aocutil::test_example;

    test_example!(example_one_1, part_one, 20, 1, 1);
    //test_example!(example_two_1, part_two, 20, 2, 1);
}
