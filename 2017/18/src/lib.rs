#[macro_use]
extern crate failure;

mod instr;
mod memory;
mod program;

use std::str::FromStr;
use std::sync::mpsc::channel;
use std::thread;

use instr::*;
use program::*;

fn parse_input(input: &str) -> Vec<Instr> {
    input.lines().map(|l| Instr::from_str(l).unwrap()).collect()
}

pub fn answer_1(input: &str) -> i64 {
    let instructions = parse_input(input);
    let (send1, recv1) = channel();
    let (send2, recv2) = channel();
    let handle = thread::spawn(move || {
        let mut p = Program::new(send1, recv2);
        loop {
            match p.execute(&instructions) {
                Ok(State::Completed) => break,
                Err(e) => {
                    println!("program error: {}", e);
                    break;
                }
                _ => (),
            }
        }
    });

    let echo = thread::spawn(move || {
        let freq = recv1.recv().unwrap();
        send2.send(freq).unwrap();

        freq
    });


    let _ = handle.join();
    echo.join().unwrap()
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
