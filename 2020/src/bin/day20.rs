use aocutil::{Grid, Point};
use std::collections::{HashMap, VecDeque};

type Vector = euclid::Vector2D<i64, euclid::UnknownUnit>;

#[derive(Clone, Debug, PartialEq)]
struct Image {
    id: i64,
    grid: Grid<char>,
}

//fn flip<T>(g: &Grid<T>) -> Grid<T>
//where
//    T: Copy + PartialEq,
//{
//    let rows = g.rows() as i64;
//    let min_x = g.iter().map(|(p, _)| p.x).min().unwrap();
//
//    g.iter()
//        .map(|(p, c)| (Point::new(p.x + rows - (2 * (p.x - min_x)), p.y), *c))
//        .collect()
//}

fn flip(g: &Grid<char>) -> Grid<char> {
    let rendered: String = g.to_string();
    let flipped: Vec<String> = rendered.lines().rev().map(|l| l.to_string()).collect();

    Grid::from(flipped.join("\n").as_str())
}

fn rotate(g: &Grid<char>) -> Grid<char> {
    let r = |p: &Point| Point::new(p.y, p.x * -1);
    g.iter().map(|(p, c)| (r(p), *c)).collect()
}

fn bottom(g: &Grid<char>) -> String {
    let (min, max) = g.bounds();

    (min.x..=max.x)
        .map(|x| g.get(&Point::new(x, min.y)).unwrap().to_owned())
        .collect()
}

fn top(g: &Grid<char>) -> String {
    let (min, max) = g.bounds();

    (min.x..=max.x)
        .map(|x| g.get(&Point::new(x, max.y)).unwrap().to_owned())
        .collect()
}

fn left(g: &Grid<char>) -> String {
    let (min, max) = g.bounds();

    (min.y..=max.y)
        .map(|y| g.get(&Point::new(min.x, y)).unwrap().to_owned())
        .collect()
}

fn right(g: &Grid<char>) -> String {
    let (min, max) = g.bounds();

    (min.y..=max.y)
        .map(|y| g.get(&Point::new(max.x, y)).unwrap().to_owned())
        .collect()
}

fn align_border(a: &Image, b: &Image) -> Option<(Point, Image)> {
    let ga = &a.grid;
    [
        b.grid.to_owned(),
        rotate(&b.grid),
        rotate(&rotate(&b.grid)),
        rotate(&rotate(&rotate(&b.grid))),
        flip(&b.grid),
        flip(&rotate(&b.grid)),
        flip(&rotate(&rotate(&b.grid))),
        flip(&rotate(&rotate(&rotate(&b.grid)))),
    ]
    .iter()
    .find_map(|gb| {
        let b_new = Image {
            id: b.id,
            grid: gb.to_owned(),
        };

        if top(&ga) == bottom(&gb) {
            return Some((Point::new(0, 1), b_new));
        }

        if bottom(&ga) == top(&gb) {
            return Some((Point::new(0, -1), b_new));
        }

        if right(&ga) == left(&gb) {
            return Some((Point::new(1, 0), b_new));
        }

        if left(&ga) == right(&gb) {
            return Some((Point::new(-1, 0), b_new));
        }

        None
    })
}

fn align(images: &[Image]) -> Vec<(Point, Image)> {
    let mut queue: VecDeque<&Image> = images.iter().collect();
    let mut aligned = vec![(Point::zero(), queue.pop_front().unwrap().to_owned())];

    'queue: while let Some(image) = queue.pop_front() {
        for b in &aligned {
            if let Some(m) = align_border(&b.1, &image) {
                aligned.push(m);
                continue 'queue;
            }
        }
        queue.push_back(&image);
    }

    aligned
}

fn stitch(images: HashMap<Point, Image>) -> Grid<char> {
    let (rows, cols) = images
        .iter()
        .next()
        .map(|(_, i)| (i.grid.rows() as i64, i.grid.cols() as i64))
        .unwrap();

    let mut complete = Grid::new();
    for (p, img) in &images {
        for (ip, icell) in img.grid.iter() {
            complete.insert(Point::new(p.x * cols + ip.x, p.y * rows + ip.y), *icell);
        }
    }

    println!("{}", complete);

    complete
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
    let aligned = align(&images);
    
    aligned.iter().for_each(|(p, image)| println!("{:?}; {}", p, image.id));

    let mut min = Point::zero();
    let mut max = Point::zero();
    for (p, _) in &aligned {
        min = min.min(*p);
        max = max.max(*p);
    }

    [
        Point::new(min.x, min.y),
        Point::new(min.x, max.y),
        Point::new(max.x, min.y),
        Point::new(max.x, max.y),
    ]
    .iter()
    .flat_map(|p| aligned.iter().find(|(ip, _)| p == ip).map(|(_, i)| i.id))
    .product::<i64>()
    .to_string()
}

fn part_two(input: &str) -> String {
    todo!();
    //    let g = stitch(align(&parse_input(input)));
    //
    //    "".to_string()
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
    test_example!(example_two_1, part_two, 20, 2, 1);

    #[test]
    fn test_align_border_normal() {
        let a = Image {
            id: 1951,
            grid: Grid::from(
                r#"#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#.."#,
            ),
        };

        let b = Image {
            id: 2311,
            grid: Grid::from(
                r#"..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###"#,
            ),
        };

        let aligned = align_border(&a, &b).unwrap();
        assert_eq!(aligned.1.id, b.id);
        assert_eq!(aligned.1.grid.to_string(), b.grid.to_string());
        assert_eq!(aligned.0, Point::new(1, 0));
    }

    #[test]
    fn test_flip() {
        let original = Grid::from(
            r#"#.##...##.
##..#.##..
##.####...
####.#.#..
.#.####...
.##..##.#.
....#..#.#
..#.#.....
####.#....
...#.#.#.#"#,
        );

        let expected = Grid::from(
            r#"...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##."#,
        );

        assert_eq!(flip(&original).to_string(), expected.to_string());
        assert_eq!(flip(&flip(&original)).to_string(), original.to_string());
    }

    #[test]
    fn test_rotate() {
        let original = Grid::from(
            r#"#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#."#,
        );

        let expected = Grid::from(
            r#".##...####
.######...
#.###.##..
#.###.###.
#.#.#.#...
.######.##
###.#..###
..#.###..#
##.##....#
..#.###..."#,
        );

        assert_eq!(rotate(&original).to_string(), expected.to_string());
        assert_eq!(
            rotate(&rotate(&rotate(&rotate(&original)))).to_string(),
            original.to_string()
        );
    }
}
