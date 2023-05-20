use crate::item::{Coal, Material, MaterialBuilder};

pub struct CoalBuilder {}

impl CoalBuilder {
    pub fn new() -> Self {
        Self {}
    }
}

impl MaterialBuilder for CoalBuilder {
    fn build(&self) -> Box<dyn Material> {
        Box::new(Coal::new())
    }
}
