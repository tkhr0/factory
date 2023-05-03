use crate::item::{Builder, Container, Fixture};
use crate::types;
use crate::Slot;

pub struct ContainerBuilder {
    name: &'static str,
    direction: types::Direction,
}

impl ContainerBuilder {
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            direction: Default::default(),
        }
    }

    pub fn build(self) -> Box<dyn Fixture> {
        Box::new(Container::<16> {
            name: self.name,
            slots: core::array::from_fn(|_| Slot::default()),
            direction: self.direction,
        })
    }
}

impl Builder for ContainerBuilder {}
