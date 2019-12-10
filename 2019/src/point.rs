#[derive(Debug, PartialEq)]
pub struct Point {
    x: f64,
    y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }

    pub fn euclidean_distance(&self, b: &Self) -> f64 {
        ((b.x - self.x).powi(2) + (b.y - self.y).powi(2)).sqrt()
    }
}
impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_euclidean_distance() {
        let a = Point::new(3., 2.);
        let b = Point::new(8., 14.);

        assert_eq!(a.euclidean_distance(&b), 13.)
    }
}
