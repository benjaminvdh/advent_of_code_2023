use std::fmt::{self, Display, Formatter};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Grid<T> {
    grid: Vec<T>,
    width: usize,
    height: usize,
}

impl<T: PartialEq> Grid<T> {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            grid: Vec::with_capacity(width * height),
            width,
            height,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> &T {
        let index = self.get_index(x, y);
        &self.grid[index]
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut T {
        let index = self.get_index(x, y);
        &mut self.grid[index]
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    fn get_index(&self, x: usize, y: usize) -> usize {
        debug_assert!(x < self.width);
        debug_assert!(y < self.height);

        y * self.width + x
    }
}

impl<T, I, II> From<II> for Grid<T>
where
    T: PartialEq,
    I: Iterator<Item = T>,
    II: Iterator<Item = I>,
{
    fn from(iter: II) -> Self {
        let mut width = 0;
        let mut height = 0;
        let mut grid = vec![];

        for iiter in iter {
            height += 1;
            width = 0;

            for item in iiter {
                width += 1;
                grid.push(item);
            }
        }

        Self {
            grid,
            width: width,
            height: height,
        }
    }
}

impl<T: Display + PartialEq> Display for Grid<T> {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), fmt::Error> {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(fmt, "{}", self.get(x, y))?;
            }
            writeln!(fmt)?;
        }

        Ok(())
    }
}
