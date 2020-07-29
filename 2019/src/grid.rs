use euclid;
use std::collections::HashMap;

pub type Point = euclid::Point2D<i64, euclid::UnknownUnit>;

#[derive(Default, Debug, PartialEq)]
pub struct Grid<T> {
    coords: HashMap<Point, T>,
    x_bounds: (i64, i64),
    y_bounds: (i64, i64),
}

impl<T> Grid<T>
where
    T: std::fmt::Display + Copy,
{
    // TODO: Make this generic over any nested iterable
    pub fn from_vec2d(v: Vec<Vec<T>>) -> Self {
        let mut g = Grid {
            coords: HashMap::default(),
            x_bounds: (0, 0),
            y_bounds: (0, 0),
        };

        for (y, row) in v.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                g.insert(Point::new(x as i64, 0 - y as i64), *cell);
            }
        }

        g
    }

    pub fn insert(&mut self, p: Point, v: T) {
        self.coords.insert(p, v);
        self.x_bounds = (self.x_bounds.0.min(p.x), self.x_bounds.1.max(p.x));
        self.y_bounds = (self.y_bounds.0.min(p.y), self.y_bounds.1.max(p.y));
    }
}

impl<T> std::fmt::Display for Grid<T>
where
    T: std::fmt::Display + std::default::Default,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in (self.y_bounds.0..=self.y_bounds.1).rev() {
            for x in self.x_bounds.0..=self.x_bounds.1 {
                if let Some(x) = self.coords.get(&Point::new(x as i64, y as i64)) {
                    write!(f, "{}", x)?;
                } else {
                    write!(f, " ")?;
                }
            }

            if y != self.y_bounds.0 {
                write!(f, "\n")?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_coords() {
        let mut g = Grid::default();
        g.insert(Point::new(0, 0), "a");
        g.insert(Point::new(2, 0), "b");
        g.insert(Point::new(1, -1), "c");
        g.insert(Point::new(0, -2), "d");
        g.insert(Point::new(2, -2), "e");

        assert_eq!(
            r#"a b
 c 
d e"#,
            format!("{}", g)
        );
    }

    #[test]
    fn from_vec2d() {
        let mut g = Grid::default();
        g.insert(Point::new(0, 0), 'a');
        g.insert(Point::new(1, 0), ' ');
        g.insert(Point::new(2, 0), 'b');
        g.insert(Point::new(0, -1), ' ');
        g.insert(Point::new(1, -1), 'c');
        g.insert(Point::new(2, -1), ' ');
        g.insert(Point::new(0, -2), 'd');
        g.insert(Point::new(1, -2), ' ');
        g.insert(Point::new(2, -2), 'e');

        assert_eq!(
            g,
            Grid::from_vec2d(vec![
                vec!['a', ' ', 'b'],
                vec![' ', 'c', ' '],
                vec!['d', ' ', 'e'],
            ])
        );
    }
}
