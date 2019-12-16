use anyhow::{anyhow, Result};
use std::char;
use std::iter;

const PATTERN: [i64; 4] = [0, 1, 0, -1];

pub struct FFTIterator {
    signal: String,
}

impl FFTIterator {
    pub fn new(signal: &str) -> Self {
        FFTIterator {
            signal: signal.to_owned(),
        }
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
    let mut signal = FFTIterator::new(input)
        .nth(99)
        .ok_or_else(|| anyhow!("oops"))?;
    signal.truncate(8);

    Ok(signal)
}

#[aoc(day16, part2)]
fn answer_2(input: &str) -> Result<String> {
    let offset = input[..7]
        .parse()
        .map_err(|e| anyhow!("unable to parse offset: {}", e))?;

    let mut digits: Vec<i64> = input
        .chars()
        .filter_map(|c| c.to_digit(10))
        .cycle()
        .take(10_000 * input.len())
        .skip(offset)
        .map(|x| x as i64)
        .collect();

    for _ in 0..100 {
        let mut sum = 0;
        for i in (0..digits.len()).rev() {
            sum += digits[i];
            digits[i] = sum.abs() % 10;
        }
    }

    Ok(digits
        .iter()
        .take(8)
        .filter_map(|&d| char::from_digit(d as u32, 10))
        .collect())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fft_iterator() {
        let mut iter = FFTIterator::new("12345678");
        assert_eq!("48226158".to_string(), iter.next().unwrap());
        assert_eq!("34040438".to_string(), iter.next().unwrap());
        assert_eq!("03415518".to_string(), iter.next().unwrap());
        assert_eq!("01029498".to_string(), iter.next().unwrap());
    }

    #[test]
    fn test_example_1() {
        assert_eq!(
            "24176176".to_owned(),
            answer_1(&input_generator("80871224585914546619083218645595")).unwrap()
        );
        assert_eq!(
            "73745418".to_owned(),
            answer_1(&input_generator("19617804207202209144916044189917")).unwrap()
        );
        assert_eq!(
            "52432133".to_owned(),
            answer_1(&input_generator("69317163492948606335995924319873")).unwrap()
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            "84462026".to_owned(),
            answer_2(&input_generator("03036732577212944063491565474664")).unwrap()
        );
        assert_eq!(
            "78725270".to_owned(),
            answer_2(&input_generator("02935109699940807407585447034323")).unwrap()
        );
        assert_eq!(
            "53553731".to_owned(),
            answer_2(&input_generator("03081770884921959731165446850517")).unwrap()
        );
    }
}
