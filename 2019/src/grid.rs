use euclid;
use std::collections::HashMap;
use std::iter;

pub trait Collision {
    fn is_collidable(&self) -> bool;
}

pub type Point = euclid::Point2D<i64, euclid::UnknownUnit>;

#[derive(Default, Debug, PartialEq, Clone)]
pub struct Grid<T> {
    coords: HashMap<Point, T>,
    x_bounds: (i64, i64),
    y_bounds: (i64, i64),
}

impl<T> Grid<T>
where
    T: Copy + PartialEq,
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

    pub fn get(&self, p: &Point) -> Option<&T> {
        self.coords.get(p)
    }

    pub fn find<'a>(&'a self, v: &'a T) -> Vec<&'a Point> {
        self.coords
            .iter()
            .filter(|(_, &x)| x == *v)
            .map(|(p, _)| p)
            .collect()
    }

    pub fn filter<P>(&self, f: P) -> impl iter::Iterator<Item = (&Point, &T)>
    where
        P: FnMut(&(&Point, &T)) -> bool,
    {
        self.coords.iter().filter(f)
    }
}

impl<T> Grid<T>
where
    T: Collision + Copy + PartialEq,
{
    pub fn shortest_path(&self, a: Point, b: Point) -> Option<Vec<Point>> {
        None
    }

    pub fn filter_surrounding<F>(&self, p: Point, f: F) -> Vec<Point>
    where
        F: Fn(&Point, &T) -> bool,
    {
        let ps = [
            Point::new(p.x - 1, p.y - 1),
            Point::new(p.x, p.y - 1),
            Point::new(p.x + 1, p.y - 1),
            Point::new(p.x - 1, p.y),
            Point::new(p.x + 1, p.y),
            Point::new(p.x - 1, p.y + 1),
            Point::new(p.x, p.y + 1),
            Point::new(p.x + 1, p.y + 1),
        ];

        ps.iter()
            .filter_map(|p| {
                let v = self.get(p)?;
                if f(p, v) {
                    Some(*p)
                } else {
                    None
                }
            })
            .collect()
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

    #[derive(Clone, Copy, Default, PartialEq)]
    struct Collidable<T>(T, bool);

    impl<T> Collision for Collidable<T> {
        fn is_collidable(&self) -> bool {
            self.1
        }
    }

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

    #[test]
    fn filter_surrounding() {
        let mut g = Grid::default();
        g.insert(Point::new(0, 0), Collidable('a', true));
        g.insert(Point::new(1, 0), Collidable(' ', false));
        g.insert(Point::new(2, 0), Collidable('b', true));
        g.insert(Point::new(0, -1), Collidable(' ', false));
        g.insert(Point::new(1, -1), Collidable('c', true));
        g.insert(Point::new(2, -1), Collidable(' ', false));
        g.insert(Point::new(0, -2), Collidable('d', true));
        g.insert(Point::new(1, -2), Collidable(' ', false));
        g.insert(Point::new(2, -2), Collidable('e', true));

        assert_eq!(
            vec![Point::new(1, -1), Point::new(0, 0), Point::new(2, 0)],
            g.filter_surrounding(Point::new(1, 0), |_, &v| v.is_collidable())
        );

        assert_eq!(
            vec![Point::new(0, -2)],
            g.filter_surrounding(Point::new(0, -3), |_, &v| v.is_collidable())
        );

        assert_eq!(
            vec![Point::new(2, -2), Point::new(2, 0)],
            g.filter_surrounding(Point::new(3, -1), |_, &v| v.is_collidable())
        );
    }
}
