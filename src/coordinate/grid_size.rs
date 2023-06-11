use std::convert::From;

#[derive(Debug)]
pub struct GridSize {
    pub width: usize,
    pub height: usize,
}

impl GridSize {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }
}

impl From<(usize, usize)> for GridSize {
    fn from((x, y): (usize, usize)) -> Self {
        Self::new(x, y)
    }
}
