use graphics::context::Context;
use graphics::Transformed;
use opengl_graphics::GlGraphics;

use crate::resource::Resource;
use crate::tile::Tile;
use crate::types;
use crate::GridSize;
use crate::Slot;
use crate::{Fixture, Iterator};

#[derive(Debug)]
pub struct Conveyer {
    #[allow(dead_code)]
    name: &'static str,
    slots: Vec<Slot>,
    cooling_time: f64,
    direction: types::Direction,
}

impl Conveyer {
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
}

impl Fixture for Conveyer {
    fn direction(&self) -> &types::Direction {
        &self.direction
    }

    fn set_direction(&mut self, direction: types::Direction) {
        self.direction = direction;
    }

    fn on_click(&mut self) {
        self.load();
    }

    fn set_cooling_time(&mut self, dt: f64) {
        self.cooling_time = dt;
    }

    fn cooling_time(&self) -> f64 {
        self.cooling_time
    }

    fn render(&self, gl: &mut GlGraphics, context: &Context) {
        const BODY: [f32; 4] = [0.749, 0.741, 0.329, 1.0];
        const RESOURCE: [f32; 4] = [0.9803, 0.9803, 0.9607, 1.0];

        let size = self.size();

        graphics::Rectangle::new(BODY).draw(
            [0.0, 0.0, size.width, size.height],
            &context.draw_state,
            context.transform,
            gl,
        );

        graphics::Polygon::new([0.3, 0.0, 0.0, 0.5]).draw(
            &[
                [0.0, 0.0 - self.height() / 2.0],
                [10.0, 10.0 - self.height() / 2.0],
                [-10.0, 10.0 - self.height() / 2.0],
            ],
            &context.draw_state,
            context
                .transform
                .trans(self.width() / 2.0, self.height() / 2.0)
                .rot_rad(self.angle()),
            gl,
        );

        for (i, slot) in self.slots.iter().enumerate() {
            if slot.is_some() {
                graphics::ellipse(
                    RESOURCE,
                    [-5.0, (-20.0 + (i as f64) * 10.0), 10.0, 10.0],
                    context
                        .transform
                        .trans(self.width() / 2.0, self.height() / 2.0)
                        .rot_rad(self.angle()),
                    gl,
                );
            }
        }
    }

    fn effect_range(&self) -> Option<GridSize> {
        Some((3, 3).into())
    }

    fn affect(&mut self, target: &mut Tile, direction: &types::Direction) {
        if direction == self.direction() && target.acceptable() {
            if let Some(resource) = self.pick() {
                target.push(Some(resource)).unwrap();
                println!(
                    "slots({}): {:?}",
                    target.fixture().unwrap().name(),
                    target.fixture().unwrap().slots()
                );
            }
        }
    }

    fn acceptable(&self) -> bool {
        if let Some(last_slot) = self.slots.last() {
            last_slot.is_empty()
        } else {
            false
        }
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

    fn slots(&self) -> &Vec<Slot> {
        &self.slots
    }

    fn name(&self) -> &'static str {
        self.name
    }
}

impl Iterator for Conveyer {
    fn iterate(&mut self) {
        println!("ITERATE({}): {:?}", self.name, self.slots);
        for i in 0..(self.slots.len() - 1) {
            if self.slots[i].is_empty() {
                self.slots.swap(i, i + 1);
            }
        }
    }
}

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
        Box::new(Conveyer {
            name: self.name,
            slots: (0..4).map(|_| Slot::default()).collect(),
            cooling_time: 0.0,
            direction,
        })
    }
}
