use graphics::context::Context;
use graphics::Transformed;
use opengl_graphics::GlGraphics;

use super::Conveyer;
use crate::coordinate;
use crate::item::{Fixture, Material};
use crate::types;
use crate::Slot;
use crate::Tile;

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
                    slot.resource().unwrap().color_symbol(),
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
        if self.operatable() && direction == self.direction() && target.pushable() {
            if let Some(resource) = self.pick() {
                target.push(Some(resource)).unwrap();
            }
        }
    }

    fn insertable(&self) -> bool {
        self.acceptable()
    }

    fn insert(&mut self, resource: Box<dyn Material>) -> Result<(), &'static str> {
        Conveyer::push(self, Some(resource))
    }

    fn pushable(&self) -> bool {
        self.acceptable()
    }

    fn push(&mut self, resource: Option<Box<dyn Material>>) -> Result<(), &'static str> {
        Conveyer::push(self, resource)
    }

    fn request(&mut self) -> Option<Box<dyn Material>> {
        for slot in self.slots.iter_mut() {
            let resource = slot.pick();
            if resource.is_some() {
                return resource;
            }
        }

        None
    }

    fn iterate(&mut self) {
        for i in 0..(self.slots.len() - 1) {
            if self.slots[i].is_empty() {
                self.slots.swap(i, i + 1);
            }
        }
    }

    fn slots(&self) -> &[Slot] {
        &self.slots
    }

    fn name(&self) -> &'static str {
        self.name
    }
}
