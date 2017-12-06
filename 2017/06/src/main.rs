fn redistributions(bank: &[i32]) -> i32 {
    0
}

fn main() {
    let bank: [i32; 4] = [0, 0, 0, 0];
    println!("{0}", redistributions(&bank));
}

#[cfg(test)]
mod tests {
    use super::redistributions;

    #[test]
    fn example() {
        let bank: [i32; 4] = [0, 2, 7, 0];
        assert_eq!(redistributions(&bank), 5);
    }
}
