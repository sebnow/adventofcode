#[macro_use]
extern crate failure;

mod moves;

use std::str;
use std::str::FromStr;
use moves::Move;

#[derive(Debug, PartialEq, Eq)]
struct Group(Vec<u8>);

impl Group {
    pub fn new(g: &[u8]) -> Self {
        Group(g.to_owned())
    }

    pub fn as_str(&self) -> &str {
        str::from_utf8(&self.0).unwrap()
    }

    pub fn spin(&mut self, pos: usize) {
        let v = {
            let (a, b) = self.0.split_at(self.0.len() - pos);
            let mut v = Vec::with_capacity(self.0.len());
            v.extend_from_slice(b);
            v.extend_from_slice(a);
            v
        };
        self.0 = v;
    }

    pub fn exchange(&mut self, a: usize, b: usize) {
        self.0.swap(a, b);
    }

    pub fn partner(&mut self, a: char, b: char) {
        let a_pos = self.0.iter().position(|&x| x == a as u8).unwrap();
        let b_pos = self.0.iter().position(|&x| x == b as u8).unwrap();
        self.0.swap(a_pos, b_pos);
    }
}


fn parse_input(input: &str) -> Vec<Move> {
    input
        .trim()
        .split(",")
        .map(|m| Move::from_str(m).unwrap())
        .collect()
}

fn dance(positions: &str, moves: &[Move]) -> String {
    let mut g = Group::new(positions.as_ref());
    for m in moves {
        match *m {
            Move::Spin(x) => g.spin(x),
            Move::Exchange(a, b) => g.exchange(a, b),
            Move::Partner(a, b) => g.partner(a, b),
        };
    }
    g.as_str().to_owned()
}

pub fn answer_1(input: &str) -> String {
    dance("abcdefghijklmnop", &parse_input(input))
}

pub fn answer_2(input: &str) -> String {
    let mut seen: Vec<String> = Vec::new();
    let reps = 1000000000;
    let mut positions = String::from("abcdefghijklmnop");
    for i in 0..reps {
        if seen.contains(&positions) {
            return seen[reps % i].to_owned();
        }
        seen.push(positions.clone());

        positions = dance(&positions, &parse_input(input));
    }
    positions
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_answer_1() {
        let mut g = Group::new("abcde".as_ref());
        g.spin(1);
        g.exchange(3, 4);
        g.partner('e', 'b');
        assert_eq!("baedc", g.as_str())
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(
            vec![Move::Spin(1), Move::Exchange(3, 4), Move::Partner('e', 'b')],
            parse_input("s1,x3/4,pe/b"),
        )
    }

    #[test]
    fn test_group_spin() {
        let mut g = Group::new("abcde".as_ref());
        g.spin(1);
        assert_eq!("eabcd", g.as_str())
    }

    #[test]
    fn test_group_exchange() {
        let mut g = Group::new("eabcd".as_ref());
        g.exchange(3, 4);
        assert_eq!("eabdc", g.as_str())
    }

    #[test]
    fn test_group_partner() {
        let mut g = Group::new("eabdc".as_ref());
        g.partner('e', 'b');
        assert_eq!("baedc", g.as_str())
    }
}
