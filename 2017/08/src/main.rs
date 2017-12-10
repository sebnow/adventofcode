use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

fn process(input: &str) -> HashMap<&str, i32> {
    let mut registers = HashMap::new();
    for instr in input.lines() {
        let default = 0;
        let mut tokens = instr.split_whitespace();
        let register = tokens.next().unwrap();
        let cmd = tokens.next().unwrap();
        let amount = tokens.next().unwrap().parse::<i32>().unwrap();
        tokens.next().unwrap();
        let target_register = tokens.next().unwrap();
        let cmp = tokens.next().unwrap();
        let bounds = tokens.next().unwrap().parse::<i32>().unwrap();

        let &target_value = registers.get(target_register).unwrap_or(&default);
        let cond = match cmp {
            ">" => target_value > bounds,
            ">=" => target_value >= bounds,
            "<" => target_value < bounds,
            "<=" => target_value <= bounds,
            "==" => target_value == bounds,
            "!=" => target_value != bounds,
            _ => false,
        };

        if !cond {
            continue;
        }

        let &value = registers.get(register).unwrap_or(&default);
        let new_value = match cmd {
            "inc" => value + amount,
            "dec" => value - amount,
            _ => value,
        };

        registers.insert(register, new_value);
    }

    registers
}

fn answer_1(input: &str) -> i32 {
    *process(input).values().max().unwrap_or(&0)
}

fn main() {
    let mut input = String::new();
    let mut file = File::open("input.txt").unwrap();
    file.read_to_string(&mut input).unwrap();

    println!("Part 1: {:?}", answer_1(&input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn examples_1() {
        let input = "b inc 5 if a > 1\na inc 1 if b < 5\nc dec -10 if a >= 1\nc inc -20 if c == 10";
        assert_eq!(answer_1(input), 1);
    }

}
