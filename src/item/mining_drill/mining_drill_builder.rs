use super::MiningDrill;
use crate::item::{Machine, MachineBuilder, Material, MaterialBuilder};
use crate::types;
use crate::Slot;

pub struct MiningDrillBuilder {
    name: &'static str,
    direction: types::Direction,
}

impl MiningDrillBuilder {
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

impl MaterialBuilder for MiningDrillBuilder {
    fn build(&self) -> Box<dyn Material> {
        Box::new(MiningDrill::<4> {
            name: self.name,
            slots: core::array::from_fn(|_| Slot::default()),
            cooling_time: 0.0,
            direction: self.direction,
        })
    }
}

impl MachineBuilder for MiningDrillBuilder {
    fn build(&self) -> Box<dyn Machine> {
        Box::new(MiningDrill::<4> {
            name: self.name,
            slots: core::array::from_fn(|_| Slot::default()),
            cooling_time: 0.0,
            direction: self.direction,
        })
    }
}
