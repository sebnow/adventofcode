use crate::Point;
use anyhow::Result;
use std::collections::HashMap;

use std::io;
use std::io::Write;

use termion::cursor::HideCursor;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;

#[derive(Debug)]
pub struct Grid<T> {
    top_left: Point<i64>,
    bottom_right: Point<i64>,
    pub points: HashMap<Point<i64>, T>,
}

impl<T> Grid<T>
where
    T: std::fmt::Display,
{
    pub fn new() -> Self {
        Grid::default()
    }

    pub fn add(&mut self, p: Point<i64>, v: T) {
        self.top_left = Point::new(self.top_left.x.min(p.x), self.top_left.y.max(p.y));
        self.bottom_right = Point::new(self.bottom_right.x.max(p.x), self.bottom_right.y.min(p.y));
        self.points.insert(p, v);
    }

    pub fn at(&self, p: Point<i64>) -> Option<&T> {
        self.points.get(&p)
    }

    pub fn render(&self) -> Result<()> {
        let stdout = io::stdout().into_raw_mode()?;
        let stdout = HideCursor::from(stdout);
        let mut stdout = AlternateScreen::from(stdout);

        write!(stdout, "{}", termion::clear::All)?;
        for (p, v) in &self.points {
            let p = Point::new(self.top_left.x + p.x + 1, self.top_left.y - p.y + 1);
            write!(
                stdout,
                "{}{}",
                termion::cursor::Goto(p.x as u16, p.y as u16),
                v
            )?;
        }

        stdout.flush()?;
        Ok(())
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Point<i64>, &T)> {
        self.points.iter()
    }
}

impl<T> Default for Grid<T> {
    fn default() -> Self {
        Grid {
            top_left: Point::default(),
            bottom_right: Point::default(),
            points: Default::default(),
        }
    }
}

impl<T> PartialEq for Grid<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.points == other.points
    }
}

impl<T> std::fmt::Display for Grid<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let height = self.top_left.y - self.bottom_right.y;
        let width = self.bottom_right.x - self.top_left.x;
        for y in 0..=height {
            for x in 0..=width {
                let p = &Point::new(self.top_left.x + x, self.top_left.y - y);
                match self.points.get(&p) {
                    Some(v) => write!(f, "{}", v),
                    None => write!(f, " "),
                }?;
            }
            if y != height {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl<T> From<&HashMap<Point<i64>, T>> for Grid<T>
where
    T: std::fmt::Display + Copy,
{
    fn from(m: &HashMap<Point<i64>, T>) -> Grid<T> {
        let mut grid = Grid::default();
        for (p, v) in m {
            grid.add(*p, *v);
        }

        grid
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_display_grid() {
        let mut g = Grid::new();

        let points = &[
            Point::new(-2, 1),
            Point::new(0, 1),
            Point::new(2, 1),
            Point::new(-1, 0),
            Point::new(1, 0),
            Point::new(-1, -1),
            Point::new(2, -1),
        ];
        for &p in points {
            g.add(p, '#');
        }

        assert_eq!(format!("{}", g), "# # #\n # # \n #  #");

        let mut g = Grid::new();

        let points = &[Point::new(0, 2), Point::new(1, 1), Point::new(1, 0)];
        for &p in points {
            g.add(p, '#');
        }

        assert_eq!(format!("{}", g), "# \n #\n #");
    }
}
