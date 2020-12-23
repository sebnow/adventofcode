use std::collections::VecDeque;

fn find_destination(cups: &VecDeque<usize>, max_label: usize) -> usize {
    let mut label = *cups.front().unwrap();
    loop {
        label = if label == 1 { max_label } else { label - 1 };
        if let Some(d) = cups.iter().position(|c| *c == label) {
            return d;
        }
    }
}

fn parse_input<'a>(input: &'a str) -> impl Iterator<Item = usize> + 'a {
    input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
}

fn part_one(input: &str) -> String {
    let mut cups: VecDeque<usize> = parse_input(input).collect();
    let mut removed: VecDeque<usize> = VecDeque::new();
    let max_label = *cups.iter().max().unwrap();

    for _ in 1..=100 {
        cups.rotate_left(1);
        for _ in 0..3 {
            removed.push_back(cups.pop_front().unwrap());
        }
        cups.rotate_right(1);

        let destination = find_destination(&cups, max_label);

        while let Some(removed_cup) = removed.pop_back() {
            cups.insert(destination + 1, removed_cup);
        }

        cups.rotate_left(1);
    }

    let pos = cups.iter().position(|c| *c == 1).unwrap();
    cups.rotate_left(pos);
    cups.pop_front();

    cups.iter().map(|c| format!("{}", c)).collect::<String>()
}

fn part_two(input: &str) -> String {
    "".to_string()
}

fn main() {
    let input = include_str!("../../input/day23.txt");
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

#[cfg(test)]
mod test {
    use super::*;
    use aocutil::test_example;

    test_example!(example_one_1, part_one, 23, 1, 1);
    //test_example!(example_two_1, part_two, 23, 2, 1);
}
