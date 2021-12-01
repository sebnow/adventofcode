use std::collections::HashMap;

//const MONSTER: [&str; 3] = [
//    "                  # ",
//    "#    ##    ##    ###",
//    " #  #  #  #  #  #   ",
//];

type Degrees = i32;
type ImageID = i64;
type PossibleAdjacent = Vec<ImageID>;
type AdjacentMap = HashMap<ImageID, Vec<PossibleAdjacent>>;
type Grid = Vec<Vec<char>>;

//trait Transform {
//    type Item;
//
//    fn transform(&mut self) -> Self::Item;
//}
//
//struct ID<I> {
//    item: I,
//}
//
//impl<I> ID<I> {
//    fn new(item: I) -> Self {
//        ID { item }
//    }
//}
//
//impl<I> Transform for ID<I> {
//    type Item = I;
//
//    fn transform(&mut self) -> Self::Item {
//        self.item
//    }
//}
//
//impl<T: std::fmt::Debug> std::fmt::Debug for ID<T> {
//    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//        f.debug_struct("ID").field("item", &self.item).finish()
//    }
//}
//
//struct Rotate<T> {
//    transform: T,
//    degrees: Degrees,
//}
//
//impl<T> Rotate<T> {
//    fn new(t: T, degrees: Degrees) -> Self {
//        Rotate {
//            transform: t,
//            degrees,
//        }
//    }
//}
//
//impl<T: Transform<Item = Grid>> Transform for Rotate<T> {
//    type Item = T::Item;
//
//    fn transform(&mut self) -> Self::Item {
//        todo!();
//    }
//}
//
//impl<T: std::fmt::Debug> std::fmt::Debug for Rotate<T> {
//    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//        f.debug_struct("Rotate")
//            .field("transform", &self.transform)
//            .field("degrees", &self.degrees)
//            .finish()
//    }
//}
//
//struct Flip<T> {
//    transform: T,
//}
//
//impl<T> Flip<T> {
//    fn new(t: T) -> Self {
//        Flip { transform: t }
//    }
//}
//
//impl<T> Transform for Flip<T>
//where
//    T: Transform<Item = Grid>,
//{
//    type Item = T::Item;
//
//    fn transform(&mut self) -> Self::Item {
//        flip(self.transform().iter())
//    }
//}
//
//impl<T: std::fmt::Debug> std::fmt::Debug for Flip<T> {
//    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//        f.debug_struct("Flip")
//            .field("transform", &self.transform)
//            .finish()
//    }
//}

//#[repr(usize)]
//enum AdjacentLocation {
//    Top,
//    Right,
//    Bottom,
//    Left,
//}
//
//impl std::ops::Index<AdjacentLocation> for Vec<AdjacentImageID> {
//    type Output = Option<i64>;
//
//    fn index(&self, index: AdjacentLocation) -> &Self::Output {
//        &self[index as usize]
//    }
//}
//
//impl std::ops::IndexMut<AdjacentLocation> for Vec<AdjacentImageID> {
//    fn index_mut(&mut self, index: AdjacentLocation) -> &mut Self::Output {
//        &mut self[index as usize]
//    }
//}

#[derive(Clone, Debug, PartialEq)]
struct Image {
    id: i64,
    grid: Grid,
}

impl Image {
    pub fn flip(&self) -> Self {
        Image {
            id: self.id,
            grid: flip(self.grid.iter()),
        }
    }

    pub fn rotate(&self, degrees: Degrees) -> Self {
        let mut g = self.grid.clone();
        let max_y = self.grid.len() - 1;
        let mut d = degrees % 360;
        if d < 0 {
            d += 360;
        }

        // TODO: Rotate arbitrary amounts properly
        for _ in 0..(d / 90) {
            for (y, xs) in self.grid.iter().enumerate() {
                for (x, &c) in xs.iter().enumerate() {
                    g[max_y - x][y] = c;
                }
            }
        }

        Image {
            id: self.id,
            grid: g,
        }
    }
}

fn map_edges(images: &[Image]) -> AdjacentMap {
    let mut map = AdjacentMap::with_capacity(images.len());

    for img in images {
        let mut matching = vec![vec![]; 4];
        let img_edges = get_edges(&img.grid);

        images
            .iter()
            .filter(|&i| !map.contains_key(&img.id) && i != img)
            .for_each(|i| {
                let i_edges = get_edges(&i.grid);

                for (dir, img_edge) in img_edges.iter().enumerate() {
                    for i_edge in &i_edges {
                        if img_edge == i_edge || *img_edge == flip(i_edge.iter()) {
                            matching[dir].push(i.id);
                        }
                    }
                }
            });

        map.insert(img.id, matching);
    }

    map
}

fn flip<'a, I, T>(x: I) -> Vec<T>
where
    I: DoubleEndedIterator<Item = &'a T>,
    T: Clone + 'a,
{
    x.into_iter().rev().map(|c| c.to_owned()).collect()
}

fn get_edges(g: &Grid) -> Vec<Vec<char>> {
    let (min_x, min_y, max_x, max_y) = (0, 0, g[0].len() - 1, g.len() - 1);

    vec![
        (min_x..=max_x).map(|x| g[max_y][x]).collect(),
        (min_y..=max_y).map(|y| g[y][max_x]).collect(),
        (min_x..=max_x).map(|x| g[min_y][x]).collect(),
        (min_y..=max_y).map(|y| g[y][min_x]).collect(),
    ]
}

fn align(images: &[Image]) -> Vec<Vec<Image>> {
    todo!();
}

fn parse_grid<'a, I>(lines: I) -> Grid
where
    I: IntoIterator<Item = &'a str>,
{
    lines.into_iter().map(|l| l.chars().collect()).collect()
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

            let grid = parse_grid(lines);

            Image { id, grid }
        })
        .collect()
}

fn part_one(input: &str) -> String {
    map_edges(&parse_input(input))
        .iter()
        .filter(|(_, e)| e.len() == 2)
        .map(|(id, _)| id)
        .product::<i64>()
        .to_string()
}

fn part_two(input: &str) -> String {
    let images = parse_input(input);
    let edges = map_edges(&images);
    println!("{:?}", edges);
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
    use itertools::Itertools;
    use pretty_assertions::assert_eq;
    use std::collections::BTreeMap;

    test_example!(example_one_1, part_one, 20, 1, 1);
    test_example!(example_two_1, part_two, 20, 2, 1);

    #[test]
    fn test_parse_grid() {
        let expected = r#"#.##...##.
##..#.##..
##.####...
####.#.#..
.#.####...
.##..##.#.
....#..#.#
..#.#.....
####.#....
...#.#.#.#"#;

        let grid = parse_grid(expected.lines());
        let rendered: String = grid.iter().map(|l| l.iter().collect::<String>()).join("\n");
        assert_eq!(rendered, expected);
    }

    //    #[test]
    //    fn test_align_border_normal() {
    //        let a = Image {
    //            id: 1951,
    //            grid: parse_grid(
    //                r#"#.##...##.
    //#.####...#
    //.....#..##
    //#...######
    //.##.#....#
    //.###.#####
    //###.##.##.
    //.###....#.
    //..#.#..#.#
    //#...##.#.."#
    //                    .lines(),
    //            ),
    //        };
    //
    //        let b = Image {
    //            id: 2311,
    //            grid: parse_grid(
    //                r#"..##.#..#.
    //##..#.....
    //#...##..#.
    //####.#...#
    //##.##.###.
    //##...#.###
    //.#.#.#..##
    //..#....#..
    //###...#.#.
    //..###..###"#
    //                    .lines(),
    //            ),
    //        };
    //
    //        let aligned = align_border(&a, &b).unwrap();
    //        assert_eq!(aligned.1.id, b.id);
    //        assert_eq!(aligned.1.grid.to_string(), b.grid.to_string());
    //        assert_eq!(aligned.0, Point::new(1, 0));
    //    }

    #[test]
    fn test_flip() {
        let original = Image {
            id: 1951,
            grid: parse_grid(
                r#"#.##...##.
##..#.##..
##.####...
####.#.#..
.#.####...
.##..##.#.
....#..#.#
..#.#.....
####.#....
...#.#.#.#"#
                    .lines(),
            ),
        };

        let expected = Image {
            id: 1951,
            grid: parse_grid(
                r#"...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##."#
                    .lines(),
            ),
        };

        assert_eq!(original.flip(), expected);
        assert_eq!(original.flip().flip(), original);
    }

    #[test]
    fn test_rotate() {
        let original = Image {
            id: 1,
            grid: parse_grid(
                r#"#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#."#
                    .lines(),
            ),
        };

        let expected = Image {
            id: original.id,
            grid: parse_grid(
                r#".##...####
.######...
#.###.##..
#.###.###.
#.#.#.#...
.######.##
###.#..###
..#.###..#
##.##....#
..#.###..."#
                    .lines(),
            ),
        };

        assert_eq!(original.rotate(90), expected);
        assert_eq!(original.rotate(270).rotate(90), original);
    }

    #[test]
    fn test_map_edges() {
        // 1951    2311    3079
        // 2729    1427    2473
        // 2971    1489    1171
        let input = include_str!("../../example/day20-01-01.txt");
        let images = parse_input(input);
        // Use a BTreeMap so that the keys are ordered, which makes the diff output actually work.
        let edges: BTreeMap<i64, Vec<Vec<ImageID>>> = map_edges(&images)
            .iter()
            .map(|(k, v)| (*k, v.clone()))
            .collect();

        let i = |id| id;

        let mut expected = BTreeMap::new();
        expected.insert(1951, vec![vec![], vec![i(2311)], vec![i(2729)], vec![]]);
        expected.insert(
            2311,
            vec![vec![], vec![i(3079)], vec![i(1427)], vec![i(1951)]],
        );
        expected.insert(3079, vec![vec![], vec![], vec![i(2473)], vec![i(2311)]]);
        expected.insert(
            2729,
            vec![vec![i(1951)], vec![i(1427)], vec![i(2971)], vec![]],
        );
        expected.insert(
            1427,
            vec![vec![i(2311)], vec![i(2473)], vec![i(1489)], vec![i(2729)]],
        );
        expected.insert(
            2473,
            vec![vec![i(3079)], vec![], vec![i(1171)], vec![i(1427)]],
        );
        expected.insert(2971, vec![vec![i(2729)], vec![i(1489)], vec![], vec![]]);
        expected.insert(
            1489,
            vec![vec![i(1427)], vec![i(1171)], vec![], vec![i(2971)]],
        );
        expected.insert(1171, vec![vec![i(2473)], vec![], vec![], vec![i(1489)]]);

        assert_eq!(edges, expected);
    }
}
