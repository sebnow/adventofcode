#[macro_use]
extern crate failure;

mod instr;
mod memory;
mod program;

use std::str::FromStr;
use std::sync::mpsc::channel;
use std::thread;

use instr::*;
use memory::*;
use program::*;

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
            &Instr::Snd(ref r) => frequency = get_value(&mem, r),
            &Instr::Set(r, ref v) => {
                let x = get_value(&mem, v);
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
            &Instr::Jgz(ref v, ref offset) => {
                let x = get_value(&mem, v);
                if x > 0 {
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

pub fn answer_2(input: &str) -> i64 {
    let instr1 = parse_input(input);
    let instr2 = parse_input(input);
    let (send1, recv1) = channel();
    let (send2, recv2) = channel();
    let h1 = thread::spawn(move || {
        let mut p = Program::new(0, send1, recv2);
        loop {
            match p.execute(&instr1) {
                Ok(State::Completed) => break,
                Err(e) => {
                    println!("program error: {}", e);
                    break;
                }
                _ => (),
            }
        }
    });

    let h2 = thread::spawn(move || {
        let mut p = Program::new(1, send2, recv1);
        loop {
            match p.execute(&instr2) {
                Ok(State::Completed) => break,
                Err(e) => {
                    println!("program error: {}", e);
                    break;
                }
                _ => (),
            }
        }
    });

    h1.join();
    h2.join();
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

    #[test]
    fn example_2() {
        let input = [
            "snd 1", "snd 2", "snd p", "rcv a", "rcv b", "rcv c", "rcv d"
        ].join("\n");
        assert_eq!(3, answer_2(&input));
    }
}
