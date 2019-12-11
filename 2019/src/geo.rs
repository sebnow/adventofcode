#[derive(Debug, PartialEq, Hash, Eq, Copy, Clone)]
pub struct Point {
    x: i64,
    y: i64,
}

impl Point {
    pub fn new(x: i64, y: i64) -> Self {
        Point { x, y }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Point::new(self.x + other.x, self.y + other.y)
    }
}

#[derive(Debug, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn to_point(&self) -> Point {
        match self {
            Direction::Up => Point::new(0, 1),
            Direction::Down => Point::new(0, -1),
            Direction::Left => Point::new(-1, 0),
            Direction::Right => Point::new(1, 0),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_euclidean_distance() {
        let a = Point::new(3, 2);
        let b = Point::new(8, 14);

        assert_eq!(a.euclidean_distance(&b), 13)
    }
}

