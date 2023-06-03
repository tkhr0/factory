use crate::NaturalResource;

pub struct IronOre {}

impl IronOre {
    const COLOR: [f32; 4] = [0.835294, 0.862745, 0.901961, 1.0];

    pub fn new() -> Self {
        Self {}
    }
}

impl NaturalResource for IronOre {
    fn color(&self) -> [f32; 4] {
        Self::COLOR
    }
}
