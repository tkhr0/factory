use crate::NaturalResource;
use crate::NaturalResourceVariant;

pub struct Coal {
    reserves: usize,
}

impl Coal {
    const COLOR: [f32; 4] = [0.9, 0.9, 0.9, 1.0];

    pub fn new() -> Self {
        Self { reserves: 10 }
    }
}

impl NaturalResource for Coal {
    fn color(&self) -> [f32; 4] {
        Self::COLOR
    }

    fn variant(&self) -> NaturalResourceVariant {
        NaturalResourceVariant::Coal
    }

    fn reserves(&self) -> usize {
        self.reserves
    }

    fn set_reserves(&mut self, amount: usize) {
        self.reserves = amount;
    }
}
