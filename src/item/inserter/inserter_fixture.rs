use graphics::context::Context;
use graphics::Transformed;
use opengl_graphics::GlGraphics;

use super::Inserter;
use crate::coordinate;
use crate::item::Fixture;
use crate::item::Material;
use crate::types;
use crate::Slot;
use crate::Tile;

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

    fn effect_range(&self) -> Option<coordinate::GridSize> {
        Some((3, 3).into())
    }

    fn affect(&mut self, target: &mut Tile, direction: &types::Direction) {
        if self.having() && self.operatable() {
            // insert
            if direction == self.direction() && target.insertable() {
                if let Some(resource) = self.pick() {
                    target.insert(resource).unwrap();
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

    fn insertable(&self) -> bool {
        false
    }

    fn insert(&mut self, _resource: Box<dyn Material>) -> Result<(), &'static str> {
        Err("No operation")
    }

    fn pushable(&self) -> bool {
        false
    }

    fn push(&mut self, resource: Option<Box<dyn Material>>) -> Result<(), &'static str> {
        Inserter::push(self, resource)
    }

    fn request(&mut self) -> Option<Box<dyn Material>> {
        None
    }

    fn iterate(&mut self) {}

    fn slots(&self) -> &[Slot] {
        &self.slots
    }

    fn name(&self) -> &'static str {
        self.name
    }
}
