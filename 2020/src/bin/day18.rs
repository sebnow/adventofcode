use std::collections::HashMap;

#[derive(Debug)]
enum Expr {
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Num(i64),
}

impl Expr {
    fn eval(&self) -> i64 {
        match self {
            Expr::Add(a, b) => a.eval() + b.eval(),
            Expr::Mul(a, b) => a.eval() * b.eval(),
            Expr::Num(n) => *n,
        }
    }
}

struct Parser<'a> {
    s: Vec<u8>,
    cursor: usize,
    max_precedence: u8,
    prec: &'a HashMap<u8, u8>,
}

impl<'a> Parser<'a> {
    pub fn new(s: &str, prec: &'a HashMap<u8, u8>) -> Self {
        let max_precedence = *prec.iter().max_by_key(|&(_, p)| p).unwrap().1;
        Parser {
            s: s.as_bytes().to_owned(),
            cursor: 0,
            max_precedence,
            prec,
        }
    }

    pub fn expr(&mut self) -> Expr {
        self.binary_operator(0)
    }

    fn binary_operator(&mut self, precedence: u8) -> Expr {
        if precedence > self.max_precedence {
            return self.operand();
        }

        let mut lhs = self.binary_operator(precedence + 1);
        while self.cursor < self.s.len() && self.next_is_operator() {
            let op = self.next();
            let op_prec = self.prec.get(&op).unwrap();
            let rhs = self.binary_operator(op_prec + 1);
            lhs = match op {
                b'+' => Expr::Add(Box::new(lhs), Box::new(rhs)),
                b'*' => Expr::Mul(Box::new(lhs), Box::new(rhs)),
                _ => unreachable!(),
            }
        }

        lhs
    }

    fn operand(&mut self) -> Expr {
        if self.peek() == b'(' {
            self.paren()
        } else {
            self.num()
        }
    }

    fn paren(&mut self) -> Expr {
        self.drop(b'(');
        let r = self.expr();
        self.drop(b')');
        r
    }

    fn num(&mut self) -> Expr {
        Expr::Num((self.next() - b'0') as i64)
    }

    fn next_is_operator(&self) -> bool {
        let c = self.peek();
        [b'+', b'*'].iter().find(|&&b| b == c).is_some()
    }

    fn peek(&self) -> u8 {
        self.s[self.cursor]
    }

    fn next(&mut self) -> u8 {
        let c = self.s[self.cursor];
        self.cursor += 1;
        c
    }

    fn drop(&mut self, _b: u8) {
        self.next();
    }
}

fn part_one(input: &str) -> String {
    let mut prec = HashMap::new();
    prec.insert(b'+', 1);
    prec.insert(b'*', 1);

    input
        .replace(" ", "")
        .lines()
        .map(|expr| Parser::new(expr, &prec).expr().eval())
        .sum::<i64>()
        .to_string()
}

fn part_two(input: &str) -> String {
    let mut prec = HashMap::new();
    prec.insert(b'+', 2);
    prec.insert(b'*', 1);

    input
        .replace(" ", "")
        .lines()
        .map(|expr| Parser::new(expr, &prec).expr().eval())
        .sum::<i64>()
        .to_string()
}

fn main() {
    let input = include_str!("../../input/day18.txt");
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

#[cfg(test)]
mod test {
    use super::*;
    use aocutil::test_example;

    test_example!(example_one_1, part_one, 18, 1, 1);
    test_example!(example_one_2, part_one, 18, 1, 2);
    test_example!(example_one_3, part_one, 18, 1, 3);
    test_example!(example_one_4, part_one, 18, 1, 4);
    test_example!(example_one_5, part_one, 18, 1, 5);
    test_example!(example_one_6, part_one, 18, 1, 5);
    test_example!(example_two_1, part_two, 18, 2, 1);
    test_example!(example_two_2, part_two, 18, 2, 2);
}
