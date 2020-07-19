fn get_input() -> Vec<char> {
    include_str!("../../input/day01.txt")
        .chars()
        .collect()
}

fn part_one(input: &[char]) -> impl std::fmt::Display {
    input.iter().map(|&x| if x == '(' {1} else {-1}).sum::<i64>()
}

fn part_two(input: &[char]) -> impl std::fmt::Display {
    let mut floor = 0;

    for (i, &c) in input.iter().enumerate() {
        floor += if c == '(' {1} else {-1};
        if floor == -1 {
            return i + 1
        }
    }
    
    return 0
}

fn main() {
    let input = get_input();
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}
