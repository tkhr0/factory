use crate::item::{Builder, Item, ItemVariant};
use crate::item::{ContainerBuilder, ConveyerBuilder};

pub struct ItemFactory {}

impl ItemFactory {
    pub fn build(variant: ItemVariant) -> Box<dyn Item> {
        let builder: Box<dyn Builder> = match variant {
            ItemVariant::Container => Box::new(ContainerBuilder::new("Container")),
            ItemVariant::Conveyer => Box::new(ConveyerBuilder::new("Conveyer")),
        };

        builder.build()
    }
}
