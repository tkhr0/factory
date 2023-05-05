use crate::item::{ContainerBuilder, ConveyerBuilder};

pub struct ItemBuilders {
    pub conveyer: ConveyerBuilder,
    pub container: ContainerBuilder,
}

impl Default for ItemBuilders {
    fn default() -> Self {
        Self {
            conveyer: ConveyerBuilder::new("ConveyerBuilder"),
            container: ContainerBuilder::new("ContainerBuilder"),
        }
    }
}
