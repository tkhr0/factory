pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }
}

pub struct Size {
    pub width: f64,
    pub height: f64,
}

impl Size {
    pub fn new(width: f64, height: f64) -> Size {
        Size { width, height }
    }
}

pub type Radian = f64;

#[derive(Debug, Default, PartialEq)]
#[rustfmt::skip]
pub enum Direction {
       #[default]
       North,
    West, East,
       South,

    None,
}

impl Direction {
    pub fn angle(&self) -> Radian {
        match self {
            Self::North => 0.0,
            Self::West => std::f64::consts::FRAC_PI_2 * 3.0,
            Self::East => std::f64::consts::FRAC_PI_2,
            Self::South => std::f64::consts::PI,
            Self::None => 0.0,
        }
    }

    pub fn next(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
            Self::None => Self::None,
        }
    }
}
