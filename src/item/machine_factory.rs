use crate::item::{ContainerBuilder, ConveyerBuilder, InserterBuilder, MiningDrillBuilder};
use crate::item::{Machine, MachineBuilder, MaterialVariant};

pub struct MachineFactory {}

impl MachineFactory {
    pub fn build(variant: MaterialVariant) -> Option<Box<dyn Machine>> {
        let builder: Box<dyn MachineBuilder> = match variant {
            MaterialVariant::Container => Box::new(ContainerBuilder::new("Container")),
            MaterialVariant::Conveyer => Box::new(ConveyerBuilder::new("Conveyer")),
            MaterialVariant::Inserter => Box::new(InserterBuilder::new("Inserter")),
            MaterialVariant::MiningDrill => Box::new(MiningDrillBuilder::new("MiningDrill")),
            _ => return None,
        };

        Some(builder.build())
    }
}
