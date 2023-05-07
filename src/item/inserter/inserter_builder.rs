use super::Inserter;
use crate::item::{Builder, Item};
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
}

impl Builder for InserterBuilder {
    fn build(&self) -> Box<dyn Item> {
        Box::new(Inserter::<1> {
            name: self.name,
            slots: core::array::from_fn(|_| Slot::default()),
            cooling_time: 0.0,
            direction: self.direction,
        })
    }
}
