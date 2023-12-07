use std::fmt::Display;

pub struct Grid<T> {
    pub data: Vec<T>,
    pub width: usize,
    pub height: usize,
}

impl<T> Grid<T>
where
    T: Clone,
{
    pub fn new(default: T, width: usize, height: usize) -> Self {
        let data = vec![default; width * height];

        Self {
            data,
            width,
            height,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        if x >= self.width || y >= self.height {
            return None;
        }

        self.data.get(y * self.width + x)
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        if x >= self.width || y >= self.height {
            return None;
        }

        self.data.get_mut(y * self.width + x)
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn iter(&self) -> GridIterator<T> {
        GridIterator {
            grid: self,
            x: 0,
            y: 0,
        }
    }

    pub fn set(&mut self, x: usize, y: usize, value: T) {
        if x >= self.width || y >= self.height {
            panic!("out of range");
        }

        self.data[y * self.width + x] = value;
    }

    pub fn width(&self) -> usize {
        self.width
    }
}

impl<T> std::fmt::Display for Grid<T>
where
    T: Clone + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for ((x, _), value) in self {
            write!(f, "{}", value)?;

            if x == self.width - 1 {
                writeln!(f)?;
            }
        }

        Ok(())
    }
}

pub struct GridIterator<'a, T> {
    grid: &'a Grid<T>,
    x: usize,
    y: usize,
}

impl<'a, T> Iterator for GridIterator<'a, T>
where
    T: Clone,
{
    type Item = ((usize, usize), T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.y >= self.grid.height() {
            return None;
        }

        let value = self.grid.get(self.x, self.y).unwrap().clone();
        let position = (self.x, self.y);

        self.x += 1;

        if self.x >= self.grid.width() {
            self.x = 0;
            self.y += 1;
        }

        Some((position, value))
    }
}

pub struct GridIntoIterator<'a, T> {
    grid: &'a Grid<T>,
    x: usize,
    y: usize,
}

impl<'a, T> Iterator for GridIntoIterator<'a, T>
where
    T: Clone,
{
    type Item = ((usize, usize), T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.y >= self.grid.height() {
            return None;
        }

        let value = self.grid.get(self.x, self.y).unwrap().clone();
        let position = (self.x, self.y);

        self.x += 1;

        if self.x >= self.grid.width() {
            self.x = 0;
            self.y += 1;
        }

        Some((position, value))
    }
}

impl<'a, T> IntoIterator for &'a Grid<T>
where
    T: Clone,
{
    type Item = ((usize, usize), T);
    type IntoIter = GridIntoIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        GridIntoIterator {
            grid: self,
            x: 0,
            y: 0,
        }
    }
}
