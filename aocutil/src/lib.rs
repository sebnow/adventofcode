mod aoc;
mod direction;
mod grid;
mod iter;
mod point;
mod test;

pub use direction::*;
pub use grid::*;
pub use iter::*;
pub use test::*;
pub use aoc::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
