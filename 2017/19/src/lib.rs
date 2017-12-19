#[macro_use]
extern crate failure;

#[derive(Debug, PartialEq, Eq)]
pub enum Sym {
    Vertical,
    Horizontal,
    Corner,
    Letter(char),
}

#[derive(Debug, Eq, PartialEq)]
pub struct Point(i64, i64);

impl Point {
    fn step(&self, other: &Point) -> Point {
        Point(self.0 + other.0, self.1 + other.1)
    }
}

fn parse_line(input: &str) -> Vec<Option<Sym>> {
    input.chars().map(|c| {
        match c {
            '|' => Some(Sym::Vertical),
            '-' => Some(Sym::Horizontal),
            '+' => Some(Sym::Corner),
            ' ' => None,
            _ => Some(Sym::Letter(c)),
        }
    }).collect()
}

fn parse_input(input: &str) -> Vec<Vec<Option<Sym>>> {
    input.lines().map(parse_line).collect()
}

fn find_start(map: &Vec<Vec<Option<Sym>>>) -> Option<Point> {
    let x = map[0].iter().position(|s| s.is_some())?;
    Some(Point(x as i64, 0))
}

fn find_paths(map: &Vec<Vec<Option<Sym>>>, &Point(x, y): &Point, &Point(lx, ly): &Point) -> Vec<Point> {
    let max_x = map[0].len() as i64;
    let max_y = map.len() as i64;

    let mut paths = Vec::with_capacity(8);
    let options = [
        (x, y + 1),
        (x - 1, y),
        (x + 1, y),
        (x, y - 1),
    ];

    for &(x, y) in options.iter() {
        if x < 0 || max_x <= x || y < 0 || max_y <= y || (lx == x && ly == y){
            continue;
        }

        let p = &map[y as usize][x as usize];
        if p.is_some() {
            paths.push(Point(x, y));
        }
    }

    paths
}

pub fn traverse(map: &Vec<Vec<Option<Sym>>>) -> Result<(String, i64), failure::Error> {
    let mut steps = 0;
    let mut letters = String::new();
    let mut dir = Point(0, 1);
    let mut p = find_start(map).ok_or(format_err!("start missing"))?;
    let mut last_p = Point(p.0, p.1);
    let (max_x, max_y) = (map[0].len() as i64, map.len() as i64);

    while 0 <= p.0 && p.0 < max_x && 0 <= p.1 && p.1 < max_y {
        match &map[p.1 as usize][p.0 as usize] {
            &Some(Sym::Letter(c)) => letters.push(c),
            &Some(Sym::Corner) => {
                let paths = find_paths(&map, &p, &last_p);
                if paths.len() > 1 {
                    return Err(format_err!("too many paths"))
                } else if paths.len() == 0 {
                    println!("No paths");
                    break;
                }

                let &Point(px, py) = paths.first().unwrap();

                dir.0 = px - p.0;
                dir.1 = py - p.1;
            },
            &Some(Sym::Vertical) | &Some(Sym::Horizontal) => {},
            &None => break,
        };

        steps += 1;
        last_p.0 = p.0;
        last_p.1 = p.1;
        p = p.step(&dir);
    }

    Ok((letters, steps))
}

pub fn answer_1(input: &str) -> Result<String, failure::Error> {
    let map = parse_input(input);
    traverse(&map).map(|r| r.0)
}

pub fn answer_2(input: &str) -> Result<i64, failure::Error> {
    let map = parse_input(input);
    traverse(&map).map(|r| r.1)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_1() {
        let input = [
            "     |          ",
            "     |  +--+    ",
            "     A  |  C    ",
            " F---|----E|--+ ",
            "     |  |  |  D ",
            "     +B-+  +--+ ",
            "                ",
        ].join("\n");

        assert_eq!(String::from("ABCDEF"), answer_1(&input).unwrap());
    }

    #[test]
    fn example_2() {
        let input = [
            "     |          ",
            "     |  +--+    ",
            "     A  |  C    ",
            " F---|----E|--+ ",
            "     |  |  |  D ",
            "     +B-+  +--+ ",
            "                ",
        ].join("\n");

        assert_eq!(38, answer_2(&input).unwrap());
    }
}
