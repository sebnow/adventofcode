use crate::point::Point;
use std::collections::HashMap;

const THRESHOLD: f64 = 0.000_1;
const PRECISION: f64 = 100_000_000.0;

#[derive(Hash, Eq, PartialEq, PartialOrd, Ord, Copy, Clone)]
struct Angle(i64);

impl Angle {
    pub fn from_f64(a: f64) -> Self {
        Angle((a * PRECISION) as i64)
    }
}

fn is_in_line_of_sight(asteroids: &[Point], a: &Point, b: &Point) -> bool {
    asteroids
        .iter()
        .filter(|&x| x != a && x != b)
        .find(|x| {
            let len_ab = a.euclidean_distance(b);
            let len_ax = a.euclidean_distance(x);
            let len_bx = b.euclidean_distance(x);

            (len_ax + len_bx) - len_ab <= THRESHOLD
        })
        .is_none()
}

fn find_best_place(asteroids: &[Point]) -> (&Point, usize) {
    let mut max = (&asteroids[0], 0);

    for a in asteroids {
        let mut visible = 0;
        for b in asteroids {
            if a == b {
                continue;
            }

            if is_in_line_of_sight(asteroids, a, b) {
                visible += 1;
                if visible > max.1 {
                    max = (&a, visible);
                }
            }
        }
    }

    max
}

fn group_by_angle(ps: &[Point], rel: &Point) -> HashMap<Angle, Vec<Point>> {
    let mut angles: Vec<(Point, Angle)> = ps
        .iter()
        .filter(|&p| p != rel)
        .map(|p| {
            let a = Angle::from_f64((p.x - rel.x).atan2(p.y - rel.y).to_degrees());
            (*p, a)
        })
        .collect();

    angles.sort_by(|(a, _), (b, _)| {
        rel.euclidean_distance(&a)
            .partial_cmp(&rel.euclidean_distance(&b))
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    let mut m = HashMap::new();
    for (p, a) in angles {
        let e = m.entry(a).or_insert_with(Vec::new);
        e.push(p);
    }

    m
}

fn sort_angles<T>(grouped: &HashMap<Angle, T>) -> Vec<Angle> {
    let mut angles = Vec::new();
    for (a, _) in grouped {
        angles.push(*a);
    }
    angles.sort();
    angles
}

fn imma_firin_mah_lazer(asteroids: &[Point], base: &Point) -> Vec<Point> {
    let mut grouped = group_by_angle(asteroids, base);
    let mut vaporized = Vec::with_capacity(asteroids.len());
    let angles = sort_angles(&grouped);

    while vaporized.len() < asteroids.len() - 1 {
        println!(
            "vaporized: {}, asteroids: {}",
            vaporized.len(),
            asteroids.len(),
        );
        for a in &angles {
            if let Some(ps) = grouped.get_mut(a) {
                if let Some(p) = ps.pop() {
                    println!("{} went poof!", p);
                    vaporized.push(p);
                }
            }
        }
    }

    vaporized
}

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<Point> {
    input
        .lines()
        .enumerate()
        .map(|(y, l)| {
            l.trim().chars().enumerate().filter_map(move |(x, p)| {
                if p == '#' {
                    Some(Point::new(x as f64, y as f64))
                } else {
                    None
                }
            })
        })
        .flatten()
        .collect()
}

#[aoc(day10, part1)]
fn answer_1(input: &[Point]) -> usize {
    let (_, visible) = find_best_place(input);
    visible
}

#[aoc(day10, part2)]
fn answer_2(input: &[Point]) -> i64 {
    let (base, _) = find_best_place(input);
    let vaporized = imma_firin_mah_lazer(input, base);
    let last = vaporized.get(200).expect("200th asteroid not found");

    (last.x * 100.0 + last.y) as i64
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn examples_1_1() {
        assert_eq!(
            8,
            answer_1(&input_generator(
                r#".#..#
.....
#####
....#
...##"#
            ))
        );
    }

    #[test]
    fn examples_1_2() {
        assert_eq!(
            33,
            answer_1(&input_generator(
                r#"......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####"#
            ))
        );
    }

    #[test]
    fn examples_1_3() {
        assert_eq!(
            35,
            answer_1(&input_generator(
                r#"#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###."#
            ))
        );
    }

    #[test]
    fn examples_1_4() {
        assert_eq!(
            41,
            answer_1(&input_generator(
                r#".#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#.."#
            ))
        );
    }

    #[test]
    fn examples_1_5() {
        assert_eq!(
            210,
            answer_1(&input_generator(
                r#".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##"#
            ))
        );
    }

    #[test]
    fn test_los() {
        let asteroids = input_generator(
            r#".#..#
.....
#####
....#
...##"#,
        );

        let best = &asteroids[8];
        assert!(!is_in_line_of_sight(&asteroids, best, &asteroids[0]),);
        assert!(is_in_line_of_sight(&asteroids, best, &asteroids[1]),);
        assert!(is_in_line_of_sight(&asteroids, best, &asteroids[2]),);
        assert!(is_in_line_of_sight(&asteroids, best, &asteroids[3]),);
        assert!(is_in_line_of_sight(&asteroids, best, &asteroids[4]),);
    }

    #[test]
    fn example_2_1() {
        let input = &input_generator(
            r#".#....#####...#..
            ##...##.#####..##
            ##...#...#.#####.
            ..#.....X...###..
            ..#.#.....#....##"#,
        );

        let (base, _) = find_best_place(&input);
        let ps = imma_firin_mah_lazer(&input, &base);

        assert_eq!(ps[0], Point { x: 8., y: 1. });
        assert_eq!(ps[1], Point { x: 9., y: 0. });
    }
}
