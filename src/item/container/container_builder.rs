use super::Container;
use crate::item::{Machine, MachineBuilder, Material, MaterialBuilder};
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
            direction: types::Direction::North,
        }
    }

    pub fn set_direction(&mut self, direction: types::Direction) {
        self.direction = direction;
    }
}

impl MaterialBuilder for ContainerBuilder {
    fn build(&self) -> Box<dyn Material> {
        Box::new(Container::<16> {
            name: self.name,
            slots: core::array::from_fn(|_| Slot::default()),
            direction: self.direction,
        })
    }
}

impl MachineBuilder for ContainerBuilder {
    fn build(&self) -> Box<dyn Machine> {
        Box::new(Container::<16> {
            name: self.name,
            slots: core::array::from_fn(|_| Slot::default()),
            direction: self.direction,
        })
    }
}
