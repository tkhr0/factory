pub type Radian = f64;

#[derive(Debug, Default, PartialEq, Clone, Copy)]
#[rustfmt::skip]
pub enum Direction {
       #[default]
          North,
    West, Origin, East,
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
            Self::Origin => 0.0,
            Self::None => 0.0,
        }
    }

    pub fn next(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
            Self::Origin => Self::None,
            Self::None => Self::None,
        }
    }

    pub fn invert(&self) -> Self {
        match self {
            Self::North => Self::South,
            Self::East => Self::West,
            Self::South => Self::North,
            Self::West => Self::East,
            Self::Origin => Self::None,
            Self::None => Self::None,
        }
    }
}
