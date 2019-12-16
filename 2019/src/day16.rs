use anyhow::{anyhow, Result};
use std::char;
use std::iter;

const PATTERN: [i64; 4] = [0, 1, 0, -1];

pub struct FFTIterator {
    signal: String,
}

impl FFTIterator {
    pub fn new(signal: String) -> Self {
        FFTIterator { signal }
    }
}

impl Iterator for FFTIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let digits: Vec<u32> = self
            .signal
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect();

        let signal: String = (1..=digits.len())
            .map(|i| {
                let pattern = PATTERN
                    .iter()
                    .map(|m| iter::repeat(m).take(i))
                    .flatten()
                    .cycle()
                    .skip(1);

                let sum: i64 = digits
                    .iter()
                    .zip(pattern)
                    .map(|(&x, &m)| x as i64 * m)
                    .sum();
                char::from_digit((sum % 10).abs() as u32, 10).unwrap()
            })
            .collect();

        self.signal = signal.to_owned();

        Some(signal)
    }
}

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> String {
    input.trim().to_owned()
}

#[aoc(day16, part1)]
fn answer_1(input: &str) -> Result<String> {
    let mut signal = FFTIterator::new(input.to_owned())
        .nth(99)
        .ok_or_else(|| anyhow!("oops"))?;
    signal.truncate(8);

    Ok(signal)
}

#[aoc(day16, part2)]
fn answer_2(input: &str) -> Result<String> {
    Ok("".to_owned())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fft_iterator() {
        let mut iter = FFTIterator::new("12345678".to_string());
        assert_eq!("48226158".to_string(), iter.next().unwrap());
        assert_eq!("34040438".to_string(), iter.next().unwrap());
        assert_eq!("03415518".to_string(), iter.next().unwrap());
        assert_eq!("01029498".to_string(), iter.next().unwrap());
    }
}
