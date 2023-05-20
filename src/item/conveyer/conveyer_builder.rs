use super::Conveyer;
use crate::item::{Machine, MachineBuilder, Material, MaterialBuilder};
use crate::types;
use crate::Slot;

pub struct ConveyerBuilder {
    name: &'static str,
    direction: types::Direction,
}

impl ConveyerBuilder {
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            direction: Default::default(),
        }
    }

    pub fn set_direction(&mut self, direction: types::Direction) {
        self.direction = direction;
    }
}

impl MaterialBuilder for ConveyerBuilder {
    fn build(&self) -> Box<dyn Material> {
        Box::new(Conveyer::<4> {
            name: self.name,
            slots: core::array::from_fn(|_| Slot::default()),
            cooling_time: 0.0,
            direction: self.direction,
        })
    }
}

impl MachineBuilder for ConveyerBuilder {
    fn build(&self) -> Box<dyn Machine> {
        Box::new(Conveyer::<4> {
            name: self.name,
            slots: core::array::from_fn(|_| Slot::default()),
            cooling_time: 0.0,
            direction: self.direction,
        })
    }
}
