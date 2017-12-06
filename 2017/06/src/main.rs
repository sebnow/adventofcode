mod bank;
use bank::Bank;

fn redistributions(bank: Bank) -> i32 {
    let mut banks = vec![bank];

    loop {
        let new_bank = banks.last().unwrap().redistribute();

        if banks.iter().find(|b| *b == &new_bank).is_some() {
            return banks.len() as i32;
        }

        banks.push(new_bank);
    }
}

fn main() {
    println!(
        "{0}",
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
