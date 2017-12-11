use std::fs::File;
use std::io::{BufRead, BufReader};

type CubeCoord = (i32, i32, i32);

#[inline]
fn cube_distance((ax, ay, az): CubeCoord, (bx, by, bz): CubeCoord) -> i32 {
    ((ax - bx).abs() + (ay - by).abs() + (az - bz).abs()) / 2
}

#[inline]
fn step((x, y, z): CubeCoord, dir: &str) -> CubeCoord {
    match dir {
        "n"  => (x  , y+1, z-1),
        "ne" => (x+1, y  , z-1),
        "se" => (x+1, y-1, z  ),
        "s"  => (x  , y-1, z+1),
        "sw" => (x-1, y  , z+1),
        "nw" => (x-1, y+1, z  ),
        _ =>    (x  , y  , z  ),
    }
}

fn parse_steps<R: BufRead>(reader: R) -> Vec<CubeCoord> {
    let mut steps = vec![(0, 0, 0)];
    let dirs = reader
        .split(b',')
        .map(|x| String::from_utf8(x.unwrap()).unwrap().trim().to_owned());

    for dir in dirs {
        let &last = steps.last().unwrap();
        steps.push(step(last, dir.as_str()));
    }

    steps
}

fn answer_1(steps: &Vec<CubeCoord>) -> i32 {
    cube_distance(*steps.first().unwrap(), *steps.last().unwrap())
}

fn answer_2(steps: &Vec<CubeCoord>) -> i32 {
    let origin = (0, 0, 0);
    steps.iter().map(|&step| cube_distance(origin, step)).max().unwrap()
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let steps = parse_steps(reader);

    println!("Part 1: {:?}", answer_1(&steps));
    println!("Part 2: {:?}", answer_2(&steps));
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn example_answer() {
        assert_eq!(answer_1(&parse_steps(Cursor::new("ne,ne,ne"))), 3);
        assert_eq!(answer_1(&parse_steps(Cursor::new("ne,ne,sw,sw"))), 0);
        assert_eq!(answer_1(&parse_steps(Cursor::new("ne,ne,s,s"))), 2);
        assert_eq!(answer_1(&parse_steps(Cursor::new("se,sw,se,sw,sw"))), 3);
    }
}
