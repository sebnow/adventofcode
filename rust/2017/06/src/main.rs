mod bank;
use bank::Bank;

#[derive(Debug)]
struct Res {
    redis_cycles: i32,
    cycles: i32,
}

fn pos(banks: &Vec<Bank>, bank: &Bank) -> Option<usize> {
    banks.iter().find(|&b| b == bank).and_then(|other| {
        banks.iter().position(|x| x == other)
    })
}

fn redistributions(bank: Bank) -> Res {
    let mut banks = vec![bank];

    loop {
        let new_bank = banks.last().unwrap().redistribute();

        if let Some(p) = pos(&banks, &new_bank) {
            return Res {
                redis_cycles: banks.len() as i32,
                cycles: (banks.len() - p) as i32,
            };
        }

        banks.push(new_bank);
    }
}

fn main() {
    println!(
        "{:?}",
        redistributions(Bank::new(
            [5, 1, 10, 0, 1, 7, 13, 14, 3, 12, 8, 10, 7, 12, 0, 6],
        ))
    );
}

#[cfg(test)]
mod tests {
    use super::redistributions;
    use bank::Bank;

    #[test]
    fn example() {
        let bank = Bank::new([0, 2, 7, 0]);
        assert_eq!(redistributions(bank), 5);
    }
}
