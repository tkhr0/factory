use graphics::context::Context;
use graphics::Transformed;
use opengl_graphics::GlGraphics;

use super::Conveyer;
use crate::item::{Fixture, Iterator};
use crate::resource::Resource;
use crate::tile::Tile;
use crate::types;
use crate::Slot;

impl<const N: usize> Fixture for Conveyer<N> {
    fn direction(&self) -> &types::Direction {
        self.direction()
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
        const RESOURCE: [f32; 4] = [0.9803, 0.9803, 0.9607, 1.0];

        let size = self.size();

        graphics::Rectangle::new(Self::COLOR_BODY).draw(
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

    fn effect_range(&self) -> Option<types::GridSize> {
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
        Conveyer::push(self, resource)
    }

    fn slots(&self) -> &[Slot] {
        &self.slots
    }

    fn name(&self) -> &'static str {
        self.name
    }
}

impl<const N: usize> Iterator for Conveyer<N> {
    fn iterate(&mut self) {
        println!("ITERATE({}): {:?}", self.name, self.slots);
        for i in 0..(self.slots.len() - 1) {
            if self.slots[i].is_empty() {
                self.slots.swap(i, i + 1);
            }
        }
    }
}