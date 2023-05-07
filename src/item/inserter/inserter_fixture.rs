use graphics::context::Context;
use graphics::Transformed;
use opengl_graphics::GlGraphics;

use super::Inserter;
use crate::item::{Fixture, Iterator};
use crate::resource::Resource;
use crate::tile::Tile;
use crate::types;
use crate::Slot;

impl<const N: usize> Fixture for Inserter<N> {
    fn direction(&self) -> &types::Direction {
        self.direction()
    }

    fn set_direction(&mut self, direction: types::Direction) {
        self.direction = direction;
    }

    fn on_click(&mut self) {}

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
        if self.having() {
            // push
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
        } else {
            // pull
            if self.acceptable() && direction == &self.direction().invert() {
                if let Some(resource) = target.request() {
                    self.push(Some(resource)).unwrap();
                }
            }
        }
    }

    fn acceptable(&self) -> bool {
        Inserter::acceptable(self)
    }

    fn push(&mut self, resource: Option<Resource>) -> Result<(), &'static str> {
        Inserter::push(self, resource)
    }

    fn request(&mut self) -> Option<Resource> {
        None
    }

    fn slots(&self) -> &[Slot] {
        &self.slots
    }

    fn name(&self) -> &'static str {
        self.name
    }
}

impl<const N: usize> Iterator for Inserter<N> {
    fn iterate(&mut self) {}
}
