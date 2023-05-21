use crate::item::{CoalBuilder, ContainerBuilder, ConveyerBuilder, InserterBuilder};
use crate::item::{Material, MaterialBuilder, MaterialVariant};

pub struct MaterialFactory {}

impl MaterialFactory {
    pub fn build(variant: MaterialVariant) -> Box<dyn Material> {
        match variant {
            MaterialVariant::Container => ContainerBuilder::new("Container").build(),
            MaterialVariant::Conveyer => ConveyerBuilder::new("Conveyer").build(),
            MaterialVariant::Inserter => InserterBuilder::new("Inserter").build(),
            MaterialVariant::Coal => CoalBuilder::new().build(),
        }
    }
}