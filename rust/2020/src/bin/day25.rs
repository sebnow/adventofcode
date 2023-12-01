const MAGIC_NUMBER: i64 = 20201227;
const SUBJECT_NUMBER: i64 = 7;

fn find_loop_size(r: i64) -> usize {
    let mut loop_size = 0;
    let mut x = 1;
    while x != r {
        x = (x * SUBJECT_NUMBER) % MAGIC_NUMBER;
        loop_size += 1;
    }

    loop_size
}

fn transform(n: i64, loop_size: usize) -> i64 {
    (0..loop_size).fold(1, |x, _| (x * n) % MAGIC_NUMBER)
}

fn parse_input<'a>(input: &'a str) -> (i64, i64) {
    let mut l = input.lines();
    (
        l.next().unwrap().parse().unwrap(),
        l.next().unwrap().parse().unwrap(),
    )
}

fn part_one(input: &str) -> String {
    let (card_pk, door_pk) = parse_input(input);
    let card_ls = find_loop_size(card_pk);
    transform(door_pk, card_ls).to_string()
}

fn part_two(input: &str) -> String {
    "".to_string()
}

fn main() {
    let input = include_str!("../../../../input/2020/day25.txt");
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

#[cfg(test)]
mod test {
    use super::*;
    use aocutil::test_example;

    test_example!(example_one_1, part_one, 25, 1, 1);
    //test_example!(example_two_1, part_two, 25, 2, 1);
}
