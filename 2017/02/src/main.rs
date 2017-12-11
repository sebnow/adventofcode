use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_line(l: String) -> Vec<i32> {
    l.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect()
}

fn parse_input<R: BufRead>(reader: R) -> Vec<Vec<i32>> {
    reader.lines().map(|l| parse_line(l.unwrap())).collect()
}

fn answer_1(input: &Vec<Vec<i32>>) -> i32 {
    input.iter().map(|xs| xs.iter().max().unwrap() - xs.iter().min().unwrap()).sum()
}

fn answer_2(input: &Vec<Vec<i32>>) -> i32 {
    input
        .iter()
        .map(|xs| {
            for &i in xs.iter() {
                for &j in xs.iter() {
                    if i == j {
                        continue
                    }
                    if i as f32 % j as f32 == 0.0 {
                        return i / j
                    }
                }
            }

            0
        })
        .sum()
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let xs = parse_input(reader);

    println!("Part 1: {:?}", answer_1(&xs));
    println!("Part 2: {:?}", answer_2(&xs));
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn example_answer_1() {
        assert_eq!(answer_1(&parse_input(Cursor::new("5 1 9 5\n7 5 3\n2 4 6 8"))), 18);
    }

    #[test]
    fn example_answer_2() {
        assert_eq!(answer_2(&parse_input(Cursor::new("5 9 2 8\n9 4 7 3\n3 8 6 5"))), 9);
    }
}
