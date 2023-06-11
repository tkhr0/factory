use std::convert::From;

#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub struct GridPoint {
    pub x: usize,
    pub y: usize,
}

impl GridPoint {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn as_index(&self, width: usize) -> usize {
        self.y * width + self.x
    }

    pub fn from_index(index: usize, width: usize) -> GridPoint {
        Self::new(index % width, index / width)
    }
}

impl From<(usize, usize)> for GridPoint {
    fn from((x, y): (usize, usize)) -> Self {
        Self::new(x, y)
    }
}
