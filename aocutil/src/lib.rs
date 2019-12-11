mod direction;
mod point;

pub use direction::*;
pub use point::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
