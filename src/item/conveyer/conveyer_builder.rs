use super::Conveyer;
use crate::item::{Builder, Item};
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

    pub fn set_direction(&mut self, direction: types::Direction) -> &mut Self {
        self.direction = direction;
        self
    }
}

impl Builder for ConveyerBuilder {
    fn build(&self) -> Box<dyn Item> {
        Box::new(Conveyer::<4> {
            name: self.name,
            slots: core::array::from_fn(|_| Slot::default()),
            cooling_time: 0.0,
            direction: self.direction,
        })
    }
}
