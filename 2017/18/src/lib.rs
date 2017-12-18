#[macro_use]
extern crate failure;

mod instr;
mod memory;

use std::str::FromStr;

use instr::*;
use memory::*;

fn parse_input(input: &str) -> Vec<Instr> {
    input.lines().map(|l| Instr::from_str(l).unwrap()).collect()
}

pub fn answer_1(input: &str) -> i64 {
    let mut pc = 0;
    let mut frequency: i64 = 0;
    let mut mem = Memory::new();
    let instructions = parse_input(input);

    loop {
        let mut jmp = 1;
        let instr = instructions.get(pc).unwrap();
        let get_value = |p: &Memory, v: &Value| match v {
            &Value::Direct(x) => x,
            &Value::Indirect(r) => p.get(r),
        };

        match instr {
            &Instr::Snd(r) => frequency = mem.get(r),
            &Instr::Set(r, ref v) => {
                let x = get_value(&Memory, v);
                mem.set(r, x);
            }
            &Instr::Add(r, ref v) => {
                let new = mem.get(r) + get_value(&mem, v);
                mem.set(r, new);
            }
            &Instr::Mul(r, ref v) => {
                let new = mem.get(r) * get_value(&mem, v);
                mem.set(r, new);
            }
            &Instr::Mod(r, ref v) => {
                let new = mem.get(r) % get_value(&mem, v);
                mem.set(r, new);
            }
            &Instr::Jgz(r, ref offset) => {
                let v = mem.get(r);
                if v > 0 {
                    jmp = get_value(&mem, offset);
                }
            }
            &Instr::Rcv(r) => {
                let v = mem.get(r);
                if v > 0 {
                    return frequency;
                }
            }
        };

        let new_pc = pc as i64 + jmp;
        if new_pc < 0 || new_pc as usize > instructions.len() {
            break;
        }
        pc = new_pc as usize;
    }

    0
}

pub fn answer_2(_input: &str) -> i64 {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_1() {
        let input = [
            "set a 1", "add a 2", "mul a a", "mod a 5", "snd a", "set a 0", "rcv a", "jgz a -1",
            "set a 1", "jgz a -2",
        ].join("\n");
        assert_eq!(4, answer_1(&input));
    }
}
