mod inserter_builder;
pub use inserter_builder::InserterBuilder;

mod inserter_fixture;
pub use inserter_fixture::*;

mod inserter_resource;
pub use inserter_resource::*;

mod inserter_sign;
pub use inserter_sign::*;

mod inserter_symbol;
pub use inserter_symbol::*;

use crate::coordinate;
use crate::item::{Machine, Material};
use crate::types;
use crate::Slot;

pub struct Inserter<const N: usize> {
    name: &'static str,
    slots: [Slot; N],
    cooling_time: f64,
    direction: types::Direction,
}

impl<const N: usize> Inserter<N> {
    const STACK_SIZE: usize = 64;
    const COLOR_BODY: types::Color = [0.929412, 0.752941, 0.270588, 1.000000];

    fn width(&self) -> f64 {
        50.0
    }

    fn height(&self) -> f64 {
        50.0
    }

    fn size(&self) -> coordinate::Size {
        coordinate::Size::new(self.width(), self.height())
    }

    fn direction(&self) -> &types::Direction {
        &self.direction
    }

    fn angle(&self) -> types::Radian {
        self.direction().angle()
    }

    fn pick(&mut self) -> Option<Box<dyn Material>> {
        if let Some(first_slot) = self.slots.first_mut() {
            first_slot.pick()
        } else {
            None
        }
    }

    fn acceptable(&self) -> bool {
        if let Some(last_slot) = self.slots.last() {
            last_slot.is_empty()
        } else {
            false
        }
    }

    fn having(&self) -> bool {
        for slot in self.slots.iter() {
            if slot.is_some() {
                return true;
            }
        }

        false
    }

    fn push(&mut self, resource: Option<Box<dyn Material>>) -> Result<(), &'static str> {
        for slot in self.slots.iter_mut() {
            if slot.is_empty() {
                return slot.push(resource);
            }
        }

        Err("No empty slot")
    }
}

impl<const N: usize> Material for Inserter<N> {}
impl<const N: usize> Machine for Inserter<N> {}
