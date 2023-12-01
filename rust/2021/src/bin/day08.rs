use std::collections::HashSet;

type Signal = HashSet<char>;

#[derive(Debug)]
struct Entry {
    signals: Vec<Signal>,
    output: Vec<Signal>,
}

fn parse_input(s: &str) -> impl Iterator<Item = Entry> + '_ {
    s.lines().map(|s| {
        let mut entry_parts = s.split('|');
        let parse = |s: &str| s.split_whitespace().map(|p| p.chars().collect()).collect();

        Entry {
            signals: entry_parts.next().map(parse).unwrap(),
            output: entry_parts.next().map(parse).unwrap(),
        }
    })
}

fn part_one(s: &str) -> String {
    let entries: Vec<_> = parse_input(s).collect();

    format!(
        "{}",
        entries
            .iter()
            .flat_map(|e| {
                e.output
                    .iter()
                    .filter(|&signals| matches!(signals.len(), 2 | 3 | 4 | 7))
            })
            .count()
    )
}

fn map_signal(
    s: &HashSet<char>,
    d1: &HashSet<char>,
    d4: &HashSet<char>,
    d7: &HashSet<char>,
) -> u32 {
    // Idea stolen from @jamescosford. So nice.
    match (
        s.len(),
        s.intersection(d1).count(),
        s.intersection(d4).count(),
        s.intersection(d7).count(),
    ) {
        (2, _, _, _) => 1,
        (5, 1, 2, 2) => 2,
        (5, 2, 3, 3) => 3,
        (4, _, _, _) => 4,
        (5, 1, 3, 2) => 5,
        (6, 1, 3, 2) => 6,
        (3, _, _, _) => 7,
        (7, _, _, _) => 8,
        (6, 2, 4, 3) => 9,
        (6, 2, 3, 3) => 0,
        _ => panic!("invalid signal"),
    }
}

fn map_output(entry: &Entry) -> u32 {
    let signal_of_len = |l: usize| {
        entry
            .signals
            .iter()
            .find(|s| s.len() == l)
            .unwrap_or_else(|| panic!("no unique signal of length {}", l))
    };

    let d1 = signal_of_len(2);
    let d4 = signal_of_len(4);
    let d7 = signal_of_len(3);

    entry
        .output
        .iter()
        .fold(0, |x: u32, signal: &HashSet<char>| {
            x * 10 + map_signal(signal, d1, d4, d7)
        })
}

fn part_two(s: &str) -> String {
    let entries: Vec<_> = parse_input(s).collect();

    format!("{}", entries.iter().map(map_output).sum::<u32>())
}

fn main() {
    let input = include_str!("../../../../input/2021/day08.txt");
    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));
}

#[cfg(test)]
mod test {
    use super::*;
    use aocutil::test_example;

    test_example!(example_8_1, part_one, 8, 1, 1);
    test_example!(example_8_2_1, part_two, 8, 2, 1);
    test_example!(example_8_2_2, part_two, 8, 2, 2);
}
