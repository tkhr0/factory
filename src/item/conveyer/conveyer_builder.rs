use super::Conveyer;
use crate::item::{Builder, Fixture};
use crate::types;
use crate::Slot;

pub struct ConveyerBuilder {
    name: &'static str,
    direction: Option<types::Direction>,
}

impl ConveyerBuilder {
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            direction: Default::default(),
        }
    }

    pub fn set_direction(&mut self, direction: types::Direction) -> &mut Self {
        self.direction = Some(direction);
        self
    }

    pub fn build(&mut self) -> Box<dyn Fixture> {
        let direction = self.direction.take().unwrap_or_default();

        Box::new(Conveyer::<4> {
            name: self.name,
            slots: core::array::from_fn(|_| Slot::default()),
            cooling_time: 0.0,
            direction,
        })
    }
}

impl Builder for ConveyerBuilder {}
