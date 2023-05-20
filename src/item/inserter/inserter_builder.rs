use super::Inserter;
use crate::item::{Machine, MachineBuilder, Material, MaterialBuilder};
use crate::types;
use crate::Slot;

pub struct InserterBuilder {
    name: &'static str,
    direction: types::Direction,
}

impl InserterBuilder {
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            direction: Default::default(),
        }
    }

    fn set_direction(&mut self, direction: types::Direction) {
        self.direction = direction;
    }
}

impl MaterialBuilder for InserterBuilder {
    fn build(&self) -> Box<dyn Material> {
        Box::new(Inserter::<1> {
            name: self.name,
            slots: core::array::from_fn(|_| Slot::default()),
            cooling_time: 0.0,
            direction: self.direction,
        })
    }
}

impl MachineBuilder for InserterBuilder {
    fn build(&self) -> Box<dyn Machine> {
        Box::new(Inserter::<1> {
            name: self.name,
            slots: core::array::from_fn(|_| Slot::default()),
            cooling_time: 0.0,
            direction: self.direction,
        })
    }
}
