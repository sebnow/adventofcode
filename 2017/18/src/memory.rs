use std::collections::HashMap;

const DEFAULT_VALUE: i64 = 0;

#[derive(Debug, Eq, PartialEq)]
pub struct Memory(HashMap<char, i64>);

impl Memory {
    pub fn new() -> Self {
        Memory(HashMap::new())
    }

    pub fn get(&self, r: char) -> i64 {
        *self.0.get(&r).unwrap_or(&DEFAULT_VALUE)
    }

    pub fn set(&mut self, r: char, v: i64) {
        self.0.insert(r, v);
    }
}
