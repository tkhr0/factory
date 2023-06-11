use std::convert::From;

use crate::coordinate::TILE_COLUMNS;

#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub struct GridPoint {
    pub x: usize,
    pub y: usize,
}

impl GridPoint {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn as_index(&self) -> usize {
        self.y * TILE_COLUMNS + self.x
    }

    pub fn from_index(index: usize) -> GridPoint {
        Self::new(index % TILE_COLUMNS, index / TILE_COLUMNS)
    }
}

impl From<(usize, usize)> for GridPoint {
    fn from((x, y): (usize, usize)) -> Self {
        Self::new(x, y)
    }
}
