use std::collections::HashMap;

const DEFAULT_VALUE: i64 = 0;

#[derive(Debug, Eq, PartialEq)]
pub struct Processor {
    registers: HashMap<char, i64>,
}

impl Processor {
    pub fn new() -> Self {
        Processor {
            registers: HashMap::new(),
        }
    }

    pub fn get(&self, r: char) -> i64 {
        *self.registers.get(&r).unwrap_or(&DEFAULT_VALUE)
    }

    pub fn set(&mut self, r: char, v: i64) {
        self.registers.insert(r, v);
    }
}
