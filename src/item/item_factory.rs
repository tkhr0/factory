use crate::item::{Builder, Item, ItemVariant};
use crate::item::{ContainerBuilder, ConveyerBuilder, InserterBuilder};

pub struct ItemFactory {}

impl ItemFactory {
    pub fn build(variant: ItemVariant) -> Box<dyn Item> {
        let builder: Box<dyn Builder> = match variant {
            ItemVariant::Container => Box::new(ContainerBuilder::new("Container")),
            ItemVariant::Conveyer => Box::new(ConveyerBuilder::new("Conveyer")),
            ItemVariant::Inserter => Box::new(InserterBuilder::new("Inserter")),
        };

        builder.build()
    }
}
