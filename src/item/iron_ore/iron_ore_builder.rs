use crate::item::{IronOre, Material, MaterialBuilder};

pub struct IronOreBuilder {}

impl IronOreBuilder {
    pub fn new() -> Self {
        Self {}
    }
}

impl MaterialBuilder for IronOreBuilder {
    fn build(&self) -> Box<dyn Material> {
        Box::new(IronOre::new())
    }
}
