use crate::resource::Resource;
use crate::types;
use crate::Slot;

mod container_builder;
pub use container_builder::*;

mod container_fixture;
pub use container_fixture::*;

pub struct Container<const N: usize> {
    name: &'static str,
    slots: [Slot; N],
    direction: types::Direction,
}

impl<const N: usize> Container<N> {
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
        let _ = self.push(Some(Resource::default()));
    }

    fn push(&mut self, resource: Option<Resource>) -> Result<(), &'static str> {
        println!("PUSH: {:?}", resource);
        println!("slots({}): {:?}", self.name, self.slots);
        if let Some(last_slot) = self.slots.last_mut() {
            last_slot.push(resource)
        } else {
            Err("No slot to push")
        }
    }
}
