#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i64> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

fn mass(x: &i64) -> i64 {
    (x / 3) - 2
}

fn mass_with_fuel(x: &i64) -> i64 {
    let mut sum = 0;
    let mut last_mass = *x;
    loop {
        let m = mass(&last_mass);
        if m <= 0 {
            return sum;
        }
        sum += m;
        last_mass = m;
    }
}

#[aoc(day1, part1)]
fn answer_1(input: &[i64]) -> i64 {
    input.iter().map(mass).sum()
}

#[aoc(day1, part2)]
fn answer_2(input: &[i64]) -> i64 {
    input.iter().map(mass_with_fuel).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn examples_2() {
        assert_eq!(2, answer_2(&vec!(14)));
        assert_eq!(966, answer_2(&vec!(1969)));
        assert_eq!(50346, answer_2(&vec!(100756)));
    }
}
