use crate::item::{ItemFactory, ItemVariant, ResourceObj};
use crate::types;
use crate::Item;
use crate::Slot;

mod container_builder;
pub use container_builder::*;

mod container_fixture;
pub use container_fixture::*;

mod container_sign;
pub use container_sign::*;

pub struct Container<const N: usize> {
    name: &'static str,
    slots: [Slot; N],
    direction: types::Direction,
}

impl<const N: usize> Container<N> {
    const COLOR_BODY: types::Color = [0.8117, 0.5019, 0.0078, 1.0];

    fn width(&self) -> f64 {
        50.0
    }

    fn height(&self) -> f64 {
        50.0
    }

    fn size(&self) -> types::Size {
        types::Size::new(self.width(), self.height())
    }

    fn direction(&self) -> &types::Direction {
        &self.direction
    }

    fn angle(&self) -> types::Radian {
        self.direction().angle()
    }

    fn load(&mut self) {
        let _ = self.push(Some(ItemFactory::build_resource(ItemVariant::Coal)));
    }

    fn push(&mut self, resource: Option<ResourceObj>) -> Result<(), &'static str> {
        if let Some(last_slot) = self.slots.last_mut() {
            last_slot.push(resource)
        } else {
            Err("No slot to push")
        }
    }
}

impl<const N: usize> Item for Container<N> {}
