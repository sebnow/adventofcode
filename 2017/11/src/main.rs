use std::fs::File;
use std::io::{BufRead, BufReader};

fn answer_1<R: BufRead>(reader: R) -> i32 {
    0
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
