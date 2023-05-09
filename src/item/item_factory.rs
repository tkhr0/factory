use crate::item::CoalBuilder;
use crate::item::{Builder, Item, ItemVariant, ResourceObj};
use crate::item::{ContainerBuilder, ConveyerBuilder, InserterBuilder};

pub struct ItemFactory {}

impl ItemFactory {
    pub fn build(variant: ItemVariant) -> Box<dyn Item> {
        let builder: Box<dyn Builder> = match variant {
            ItemVariant::Container => Box::new(ContainerBuilder::new("Container")),
            ItemVariant::Conveyer => Box::new(ConveyerBuilder::new("Conveyer")),
            ItemVariant::Inserter => Box::new(InserterBuilder::new("Inserter")),
            // HACK
            _ => panic!("Not Implemented"),
        };

        builder.build()
    }

    // HACK
    pub fn build_resource(variant: ItemVariant) -> ResourceObj {
        match variant {
            ItemVariant::Coal => return CoalBuilder::new().build_resource(),
            _ => panic!("Not Implemented"),
        }
    }
}
