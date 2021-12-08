use std::collections::{HashMap, HashSet};

// Amount of segments -> Possible Digits
// 1 -> []
// 2 -> [1]
// 3 -> [7]
// 4 -> [4]
// 5 -> [2, 3, 5]
// 6 -> [0, 6, 9]
// 7 -> 8

// Digit -> Segments
// 0 -> [A, B, C, E, F, G]
// 1 -> [C, F]
// 2 -> [A, C, D, E, F]
// 3 -> [A, C, D, F, G]
// 4 -> [B, C, D, F]
// 5 -> [A, B, D, F, G]
// 6 -> [A, B, D, E, F, G]
// 7 -> [A, C, F]
// 8 -> [A, B, C, D, E, F, G]
// 9 -> [A, B, C, D, F, G]

type Signal = Vec<char>;

#[derive(Debug)]
struct Entry {
    signals: Vec<Signal>,
    output: Vec<Signal>,
}

#[derive(Debug, Copy, Clone)]
enum Segment {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

//fn segments_to_digit(s: &[Segment]) -> u32 {
//    use Segment::*;
//
//    match s {
//        [A, B, C, E, F, G] => 0,
//        [C, F] => 1,
//        [A, C, D, E, F] => 3,
//        [B, C, D, F] => 4,
//        [A, B, D, F, G] => 5,
//        [A, B, D, E, F, G] => 6,
//        [A, C, F] => 7,
//        [A, B, C, D, E, F, G] => 8,
//        [A, B, C, D, F, G] => 9,
//        _ => panic!("invalid segments: {:?}", s),
//    }
//}

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
                e.output.iter().filter(|&signals| match signals.len() {
                    2 | 3 | 4 | 7 => true,
                    _ => false,
                })
            })
            .count()
    )
}

// TODO: This is ridiculous. Find an algorithm, dummy.
fn detect_wires(entry: &Entry) -> Vec<(HashSet<char>, u32)> {
    let signals: Vec<HashSet<char>> = entry
        .signals
        .iter()
        .map(|v| v.iter().copied().collect())
        .collect();
    let mut digits: HashMap<u32, HashSet<char>> = HashMap::with_capacity(9);
    //let mut wires: HashMap<char, Segment> = HashMap::with_capacity(9);

    let signal_of_len = |l: usize| {
        signals
            .iter()
            .find(|s| s.len() == l)
            .expect(&format!("no unique signal of length {}", l))
            .clone()
    };

    digits.insert(1, signal_of_len(2));
    digits.insert(7, signal_of_len(3));
    digits.insert(4, signal_of_len(4));
    digits.insert(8, signal_of_len(7));

    // 3 = len == 5 && 1
    {
        let one = digits.get(&1).unwrap();
        let s3 = signals
            .iter()
            .find(|s| s.len() == 5 && one.is_subset(s))
            .unwrap()
            .clone();

        println!("3: {:?}", s3);
        digits.insert(3, s3);
    }

    // 9 = len == 6 && 4
    {
        let four = digits.get(&4).unwrap();
        let s9 = signals
            .iter()
            .find(|s| s.len() == 6 && four.is_subset(s))
            .unwrap()
            .clone();

        println!("9: {:?}", s9);
        digits.insert(9, s9);
    }

    // 6 = len == 6 && !1
    {
        let one = digits.get(&1).unwrap();
        let s6 = signals
            .iter()
            .find(|s| s.len() == 6 && !one.is_subset(s))
            .unwrap()
            .clone();

        // 5 = 6 - E
        //let s5 = &s6 - &e_unique_segments;
        //digits.insert(5, s5);

        println!("6: {:?}", s6);
        digits.insert(6, s6);
    }

    // 5 = len == 5 && 6
    {
        let six = digits.get(&6).unwrap();
        let s5 = signals
            .iter()
            .find(|s| s.len() == 5 && six.is_superset(s))
            .unwrap()
            .clone();

        println!("--> 6: {:?}", six);
        println!("5: {:?}", s5);
        digits.insert(5, s5);
    }

    // 2 = len == 5 && !3 && !5
    {
        let three = digits.get(&3).unwrap();
        let five = digits.get(&5).unwrap();
        let s2 = signals
            .iter()
            .find(|s| s.len() == 5 && s != &three && s != &five)
            .unwrap()
            .clone();

        println!("2: {:?}", s2);
        digits.insert(2, s2);
    }

    // 7 - 1 = A
    //let a_unique_segments = digits.get(&7).unwrap() - digits.get(&1).unwrap();
    //let seg_a = *a_unique_segments.iter().next().unwrap();
    //wires.insert(seg_a, Segment::A);

    // G = 9 - (4 + a)
    //    let mut a4 = digits.get(&4).unwrap().clone();
    //    a4.insert(seg_a);
    //    for s in &signals {
    //        let diff = s - &a4;
    //        if a4.is_subset(s) && diff.len() == 1 {
    //            let seg_g = *diff.iter().next().unwrap();
    //            wires.insert(seg_g, Segment::G);
    //            digits.insert(9, s.clone());
    //            break;
    //        }
    //    }

    // E = 9 - 8
    //let e_unique_segments = digits.get(&8).unwrap() - digits.get(&9).unwrap();
    //let seg_e = *e_unique_segments.iter().next().unwrap();
    //wires.insert(seg_e, Segment::E);

    // 0 = len == 6 && 1 && !3
    // 0 = len == 6 && !5
    {
        let five = digits.get(&5).unwrap();
        let s0 = signals
            .iter()
            .find(|s| s.len() == 6 && !s.is_superset(five))
            .unwrap()
            .clone();
        println!("0: {:?}", s0);
        digits.insert(0, s0);
    }

    // B = 9 - 3
    //let b_unique_segments = digits.get(&9).unwrap() - digits.get(&3).unwrap();
    //wires.insert(b_unique_segments.iter().next().unwrap().clone(), Segment::B);

    println!("Digits[{}/10]: {:?}", digits.len(), digits);
    //println!("Wires [{}/10]: {:?}", wires.len(), wires);
    digits.iter().map(|(k, v)| (v.clone(), *k)).collect()
}

fn get_output(entry: &Entry) -> u32 {
    let wires = detect_wires(entry);

    entry.output.iter().fold(0, |x: u32, signal: &Signal| {
        let set: HashSet<char> = signal.iter().copied().collect();
        //println!("looking for {:?} ({:?}) in {:?}", signal, set, wires);
        let digit = wires.iter().find(|(s, _)| s == &set).unwrap().1;
        x * 10 + digit
    })
}

fn part_two(s: &str) -> String {
    let entries: Vec<_> = parse_input(s).collect();

    format!("{}", entries.iter().map(get_output).sum::<u32>())
}

fn main() {
    let input = include_str!("../../input/day08.txt");
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
