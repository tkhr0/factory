#[derive(Default)]
pub struct GridPoint {
    pub x: usize,
    pub y: usize,
}

impl GridPoint {
    pub fn new(x: usize, y: usize) -> GridPoint {
        GridPoint { x, y }
    }

    pub fn to_index(&self, width: usize) -> usize {
        self.y * width + self.x
    }
}
