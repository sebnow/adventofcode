#[derive(Eq, PartialEq, Debug)]
pub struct Bank([i32; 16]);

impl Bank {
    pub fn new(arr: [i32; 16]) -> Self {
        Bank(arr)
    }

    pub fn redistribute(&self) -> Self {
        let mut idx = self.position_of_max();
        let mut bank = self.0.clone();

        let mut blocks = bank[idx];
        bank[idx] = 0;

        while blocks > 0 {
            idx = idx + 1;
            if idx >= bank.len() {
                idx = 0;
            }

            bank[idx] += 1;
            blocks -= 1;
        }

        Bank(bank)
    }

    fn position_of_max(&self) -> usize {
        let max = self.0.iter().max().unwrap();
        self.0.iter().position(|x| x == max).unwrap()
    }
}
