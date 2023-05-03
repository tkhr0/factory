mod conveyer_builder;
pub use conveyer_builder::ConveyerBuilder;

mod conveyer_fixture;
pub use conveyer_fixture::*;

mod conveyer_sign;
pub use conveyer_sign::*;

use crate::resource::Resource;
use crate::types;
use crate::Slot;

#[derive(Debug)]
pub struct Conveyer<const N: usize> {
    #[allow(dead_code)]
    name: &'static str,
    slots: [Slot; N],
    cooling_time: f64,
    direction: types::Direction,
}

impl<const N: usize> Conveyer<N> {
    pub const COLOR_BODY: types::Color = [0.749, 0.741, 0.329, 1.0];

    fn direction(&self) -> &types::Direction {
        &self.direction
    }

    pub fn load(&mut self) {
        if let Some(last_slot) = self.slots.last_mut() {
            let _ = last_slot.push(Some(Resource::default()));
        }
    }

    fn pick(&mut self) -> Option<Resource> {
        if let Some(first_slot) = self.slots.first_mut() {
            first_slot.pick()
        } else {
            None
        }
    }

    fn width(&self) -> f64 {
        50.0
    }

    fn height(&self) -> f64 {
        50.0
    }

    fn size(&self) -> types::Size {
        types::Size::new(self.width(), self.height())
    }

    fn angle(&self) -> types::Radian {
        self.direction().angle()
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
