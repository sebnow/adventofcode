use euclid::{UnknownUnit, Vector2D};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Into<Vector2D<i64, UnknownUnit>> for Direction {
    fn into(self) -> Vector2D<i64, UnknownUnit> {
        use Direction::*;

        match self {
            Up => Vector2D::new(0, 1),
            Down => Vector2D::new(0, -1),
            Left => Vector2D::new(-1, 0),
            Right => Vector2D::new(1, 0),
        }
    }
}
