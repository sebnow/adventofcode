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

fn answer_1<R: BufRead>(reader: R) -> i32 {
    let origin = (0, 0, 0);
    cube_distance(origin, reader
        .split(b',')
        .map(|x| String::from_utf8(x.unwrap()).unwrap().trim().to_owned())
        .fold(origin, |pos, dir| step(pos, dir.as_str())))
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    println!("Part 1: {:?}", answer_1(reader))
}


#[cfg(test)]
mod test {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn example_answer() {
        assert_eq!(answer_1(Cursor::new("ne,ne,ne")), 3);
        assert_eq!(answer_1(Cursor::new("ne,ne,sw,sw")), 0);
        assert_eq!(answer_1(Cursor::new("ne,ne,s,s")), 2);
        assert_eq!(answer_1(Cursor::new("se,sw,se,sw,sw")), 3);
    }
}
