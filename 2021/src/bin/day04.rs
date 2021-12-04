use anyhow::{anyhow, Result};
use aocutil::Grid;

#[derive(PartialEq, Copy, Clone)]
enum BingoCell {
    Marked(u32),
    Unmarked(u32),
}

type Board = Vec<Vec<BingoCell>>;

struct Input {
    numbers: Vec<u32>,
    boards: Vec<Board>,
}

fn parse_board(s: &str) -> Board {
    s.lines()
        .map(|l| {
            l.split_whitespace()
                .map(|x| BingoCell::Unmarked(x.parse().unwrap()))
                .collect()
        })
        .collect()
}

fn parse_input(s: &str) -> Input {
    let mut parts = s.split("\n\n");

    let numbers: Vec<u32> = parts
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();

    let boards: Vec<Board> = parts.map(parse_board).collect();

    Input { numbers, boards }
}

fn has_bingo(b: &Board) -> bool {
    let row_bingo = (0..b.len()).any(|y| {
        (0..b[y].len()).all(|x| match b[y][x] {
            BingoCell::Marked(_) => true,
            _ => false,
        })
    });

    let col_bingo = (0..b[0].len()).any(|x| {
        (0..b[x].len()).all(|y| match b[y][x] {
            BingoCell::Marked(_) => true,
            _ => false,
        })
    });

    row_bingo || col_bingo
}

fn score(b: &Board) -> u32 {
    b.iter()
        .flat_map(|row| {
            row.iter().filter_map(|&c| match c {
                BingoCell::Unmarked(x) => Some(x),
                _ => None,
            })
        })
        .sum()
}

fn wins(s: &str) -> Vec<(Board, u32)> {
    let mut input = parse_input(s);
    let mut wins = Vec::with_capacity(input.boards.len());

    for drawn_number in input.numbers {
        for board_num in 0..input.boards.len() {
            if has_bingo(&input.boards[board_num]) {
                continue;
            }

            for y in 0..input.boards[board_num].len() {
                for x in 0..input.boards[board_num][y].len() {
                    if input.boards[board_num][y][x] == BingoCell::Unmarked(drawn_number) {
                        input.boards[board_num][y][x] = BingoCell::Marked(drawn_number);
                    }
                }
            }

            if has_bingo(&input.boards[board_num]) {
                wins.push((input.boards[board_num].clone(), drawn_number));
            }
        }
    }

    wins
}

fn part_one(s: &str) -> String {
    let wins = wins(s);
    format!("{}", score(&wins[0].0) * wins[0].1)
}

fn part_two(s: &str) -> String {
    let wins = wins(s);
    let last_win = wins.iter().last().unwrap();

    format!("{}", score(&last_win.0) * last_win.1)
}

fn main() -> Result<()> {
    let input = include_str!("../../input/day04.txt");
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use aocutil::test_example;

    test_example!(example_4_1, part_one, 4, 1, 1);
    test_example!(example_4_2, part_two, 4, 2, 1);

    #[test]
    fn marked_row() {
        let board = vec![
            vec![BingoCell::Marked(14), BingoCell::Marked(21), BingoCell::Marked(17), BingoCell::Marked(24), BingoCell::Marked(4)],
            vec![BingoCell::Unmarked(10), BingoCell::Unmarked(16), BingoCell::Unmarked(15), BingoCell::Unmarked(9), BingoCell::Unmarked(19)],
            vec![BingoCell::Unmarked(18), BingoCell::Unmarked(8), BingoCell::Unmarked(23), BingoCell::Unmarked(26), BingoCell::Unmarked(20)],
            vec![BingoCell::Unmarked(22), BingoCell::Unmarked(11), BingoCell::Unmarked(13), BingoCell::Unmarked(6), BingoCell::Unmarked(5)],
            vec![BingoCell::Unmarked(2), BingoCell::Unmarked(0), BingoCell::Unmarked(12), BingoCell::Unmarked(3), BingoCell::Unmarked(7)],
        ];

        assert!(has_bingo(&board));
    }
}
