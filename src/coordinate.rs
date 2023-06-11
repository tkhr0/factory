mod grid_point;
pub use grid_point::*;

mod grid_size;
pub use grid_size::*;

mod point;
pub use point::*;

mod size;
pub use size::*;

pub const TILE_COLUMNS: usize = 16;
pub const TILE_ROWS: usize = 16;
pub const TILE_LENGTH: usize = TILE_COLUMNS * TILE_ROWS;
pub const TILE_SIZE: f64 = 50.0;
