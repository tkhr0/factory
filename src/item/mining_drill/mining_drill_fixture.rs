use graphics::context::Context;
use graphics::Transformed;
use opengl_graphics::GlGraphics;

use super::MiningDrill;
use crate::item::Fixture;
use crate::item::Material;
use crate::types;
use crate::Slot;
use crate::Tile;

impl<const N: usize> Fixture for MiningDrill<N> {
    fn direction(&self) -> &types::Direction {
        &self.direction
    }

    fn set_direction(&mut self, direction: types::Direction) {
        self.direction = direction
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

    fn on_click(&mut self) {}

    fn cooling_time(&self) -> f64 {
        self.cooling_time
    }

    fn set_cooling_time(&mut self, dt: f64) {
        self.cooling_time = dt;
    }

    fn effect_range(&self) -> Option<types::GridSize> {
        Some((1, 1).into())
    }

    fn iterate(&mut self) {
        // TODO
    }

    fn affect(&mut self, _target: &mut Tile, _direction: &types::Direction) {
        // TODO
    }

    fn insertable(&self) -> bool {
        false
    }

    fn insert(&mut self, _resource: Box<dyn Material>) -> Result<(), &'static str> {
        Err("Not implement")
    }

    fn pushable(&self) -> bool {
        false
    }

    fn push(&mut self, _resource: Option<Box<dyn Material>>) -> Result<(), &'static str> {
        Err("Not implement")
    }

    fn request(&mut self) -> Option<Box<dyn Material>> {
        None
    }

    fn slots(&self) -> &[Slot] {
        &self.slots
    }

    fn name(&self) -> &'static str {
        self.name
    }
}
