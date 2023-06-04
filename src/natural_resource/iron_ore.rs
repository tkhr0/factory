use crate::NaturalResource;
use crate::NaturalResourceVariant;

pub struct IronOre {
    reserves: usize,
}

impl IronOre {
    const COLOR: [f32; 4] = [0.835294, 0.862745, 0.901961, 1.0];

    pub fn new() -> Self {
        Self { reserves: 10 }
    }
}

impl NaturalResource for IronOre {
    fn color(&self) -> [f32; 4] {
        Self::COLOR
    }

    fn variant(&self) -> NaturalResourceVariant {
        NaturalResourceVariant::IronOre
    }

    fn reserves(&self) -> usize {
        self.reserves
    }

    fn set_reserves(&mut self, amount: usize) {
        self.reserves = amount;
    }
}
