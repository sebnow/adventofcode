use aocutil::{self, EuclideanDistance};

type Point = aocutil::Point<f64>;

fn is_in_line_of_sight(asteroids: &[Point], a: &Point, b: &Point) -> bool {
    asteroids
        .iter()
        .filter(|&x| x != a && x != b)
        .find(|x| {
            let len_ab = a.euclidean_distance(b);
            let len_ax = a.euclidean_distance(x);
            let len_bx = b.euclidean_distance(x);

            (len_ax + len_bx) - len_ab <= 0.000_1
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

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<Point> {
    input
        .lines()
        .enumerate()
        .map(|(y, l)| {
            l.chars().enumerate().filter_map(move |(x, p)| {
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
fn answer_2(_input: &[Point]) -> usize {
    0
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
}
