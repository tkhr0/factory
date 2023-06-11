mod container_builder;
pub use container_builder::*;

mod container_fixture;
pub use container_fixture::*;

mod container_resource;
pub use container_resource::*;

mod container_sign;
pub use container_sign::*;

mod container_symbol;
pub use container_symbol::*;

use crate::coordinate;
use crate::item::{Machine, Material, MaterialFactory, MaterialVariant};
use crate::types;
use crate::Slot;

pub struct Container<const N: usize> {
    name: &'static str,
    slots: [Slot; N],
    direction: types::Direction,
}

impl<const N: usize> Container<N> {
    const STACK_SIZE: usize = 64;
    const COLOR_BODY: types::Color = [0.8117, 0.5019, 0.0078, 1.0];

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

    fn load(&mut self) {
        let _ = self.push(Some(MaterialFactory::build(MaterialVariant::Coal)));
    }

    fn push(&mut self, resource: Option<Box<dyn Material>>) -> Result<(), &'static str> {
        if let Some(last_slot) = self.slots.last_mut() {
            last_slot.push(resource)
        } else {
            Err("No slot to push")
        }
    }
}

impl<const N: usize> Material for Container<N> {}
impl<const N: usize> Machine for Container<N> {}
