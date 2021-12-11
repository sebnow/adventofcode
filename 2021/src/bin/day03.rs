fn part_one(s: &str) -> String {
    let report: Vec<Vec<char>> = s.lines().map(|l| l.chars().collect()).collect();

    let mut gamma: u32 = 0;
    let mut epsilon: u32 = 0;

    for i in 0..report[0].len() {
        let sum: u32 = report.iter().map(|r| r[i].to_digit(10).unwrap()).sum();

        if sum as usize > report.len() / 2 {
            gamma = (gamma << 1) + 1;
            epsilon <<= 1;
        } else {
            gamma <<= 1;
            epsilon = (epsilon << 1) + 1;
        }
    }

    format!("{}", gamma * epsilon)
}

fn count_set(report: &[&Vec<char>], pos: usize) -> usize {
    report
        .iter()
        .map(|r| match r[pos] {
            '1' => 1,
            '0' => 0,
            _ => panic!("invalid value {}", r[pos]),
        })
        .sum::<u32>() as usize
}

fn o_rating(report: &[Vec<char>]) -> u32 {
    let mut report: Vec<_> = report.iter().collect();

    for pos in 0..report[0].len() {
        let amount_set = count_set(&report, pos);
        let most_common = if amount_set >= report.len() - amount_set {
            '1'
        } else {
            '0'
        };
        report.retain(|n| n[pos] == most_common);

        if report.len() == 1 {
            let num: String = report[0].iter().collect();
            return u32::from_str_radix(&num, 2).unwrap();
        }
    }

    0
}

fn co2_rating(report: &[Vec<char>]) -> u32 {
    let mut report : Vec<_>= report.iter().collect();

    for pos in 0..report[0].len() {
        let amount_set = count_set(&report, pos);
        let least_common = if amount_set < report.len() - amount_set {
            '1'
        } else {
            '0'
        };
        report.retain(|n| n[pos] == least_common);

        if report.len() == 1 {
            let num: String = report[0].iter().collect();
            return u32::from_str_radix(&num, 2).unwrap();
        }
    }

    0
}

fn part_two(s: &str) -> String {
    let report: Vec<Vec<char>> = s.lines().map(|l| l.chars().collect()).collect();

    format!("{}", o_rating(&report) * co2_rating(&report))
}

fn main() {
    let input = include_str!("../../input/day03.txt");
    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));
}

#[cfg(test)]
mod test {
    use super::*;
    use aocutil::test_example;

    test_example!(example_3_1, part_one, 3, 1, 1);
    test_example!(example_3_2, part_two, 3, 2, 1);
}
