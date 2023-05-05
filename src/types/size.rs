use piston::Size as pistonSize;

#[derive(Clone, Copy)]
pub struct Size {
    pub width: f64,
    pub height: f64,
}

impl Size {
    pub const fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }
}

impl From<Size> for pistonSize {
    fn from(val: Size) -> Self {
        piston::Size {
            width: val.width,
            height: val.height,
        }
    }
}
