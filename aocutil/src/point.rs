use crate::direction::Direction;
use num_traits::{one, zero, One, Signed, Zero};

pub trait EuclideanDistance<T> {
    fn euclidean_distance(&self, other: &Self) -> T;
}

#[derive(Debug, PartialEq, Hash, Eq, Copy, Clone)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
}

impl<T> Default for Point<T>
where
    T: Default,
{
    fn default() -> Self {
        Point::new(Default::default(), Default::default())
    }
}

impl<T> std::fmt::Display for Point<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl<T> std::ops::Add for Point<T>
where
    T: std::ops::Add<Output = T>,
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Point::new(self.x + other.x, self.y + other.y)
    }
}

impl EuclideanDistance<f64> for Point<f64> {
    fn euclidean_distance(&self, other: &Self) -> f64 {
        ((other.x - self.x).powi(2) + (other.y - self.y).powi(2)).sqrt()
    }
}

impl EuclideanDistance<i64> for Point<i64> {
    fn euclidean_distance(&self, other: &Self) -> i64 {
        ((other.x - self.x).pow(2) as f64 + (other.y - self.y).pow(2) as f64).sqrt() as i64
    }
}

impl<T> From<Direction> for Point<T>
where
    T: Signed + Zero + One + std::ops::Neg,
{
    fn from(d: Direction) -> Self {
        match d {
            Direction::Up => Point::new(zero(), one()),
            Direction::Down => Point::new(zero(), one::<T>().neg()),
            Direction::Left => Point::new(one::<T>().neg(), zero()),
            Direction::Right => Point::new(one(), zero()),
        }
    }
}

impl<T> From<(T, T)> for Point<T> {
    fn from((x, y): (T, T)) -> Self {
        Point::new(x, y)
    }
}

impl<T> Into<(T, T)> for Point<T> {
    fn into(self) -> (T, T) {
        (self.x, self.y)
    }
}

impl<T> From<[T; 2]> for Point<T>
where
    T: Copy,
{
    fn from(v: [T; 2]) -> Self {
        Point::new(v[0], v[1])
    }
}

impl<T> Into<[T; 2]> for Point<T> {
    fn into(self) -> [T; 2] {
        [self.x, self.y]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_point_from_direction() {
        assert_eq!(Point::from(Direction::Up), Point::new(0, 1));
        assert_eq!(Point::from(Direction::Down), Point::new(0, -1));
        assert_eq!(Point::from(Direction::Right), Point::new(1, 0));
        assert_eq!(Point::from(Direction::Left), Point::new(-1, 0));
    }

    #[test]
    fn test_euclidean_distance() {
        assert_eq!(Point::new(3, 2).euclidean_distance(&Point::new(8, 14)), 13);
        assert!(
            (Point::new(3.0, 2.0).euclidean_distance(&Point::new(8.0, 14.0)) - 13.0).abs() < 0.0001
        );
    }
}
