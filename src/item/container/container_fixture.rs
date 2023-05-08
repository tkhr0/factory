use graphics::context::Context;
use graphics::Transformed;
use opengl_graphics::GlGraphics;

use super::Container;
use crate::item::Fixture;
use crate::types;
use crate::Resource;
use crate::Slot;
use crate::Tile;

impl<const N: usize> Fixture for Container<N> {
    fn direction(&self) -> &types::Direction {
        &self.direction
    }

    fn set_direction(&mut self, direction: types::Direction) {
        self.direction = direction;
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
                    [
                        2.0 + 12.0 * (i as f64 % 4.0),
                        2.0 + 12.0 * (i / 4) as f64,
                        10.0,
                        10.0,
                    ],
                    context.transform,
                    gl,
                );
            }
        }
    }

    fn on_click(&mut self) {
        self.load();
    }

    fn update(&mut self, _dt: f64) {}

    fn operatable(&self) -> bool {
        true
    }

    fn set_cooling_time(&mut self, _dt: f64) {}
    fn cooling_time(&self) -> f64 {
        0.0
    }

    fn effect_range(&self) -> Option<types::GridSize> {
        None
    }
    fn affect(&mut self, _target: &mut Tile, _direction: &types::Direction) {}

    fn insertable(&self) -> bool {
        self.slots.iter().any(|slot| slot.is_empty())
    }

    fn insert(&mut self, resource: Resource) -> Result<(), &'static str> {
        for slot in self.slots.iter_mut() {
            if slot.is_empty() {
                return slot.push(Some(resource));
            }
        }

        Err("No empty slot")
    }

    fn pushable(&self) -> bool {
        false
    }

    fn push(&mut self, _resource: Option<Resource>) -> Result<(), &'static str> {
        Err("No operation")
    }

    fn request(&mut self) -> Option<Resource> {
        for slot in self.slots.iter_mut() {
            let resource = slot.pick();
            if resource.is_some() {
                return resource;
            }
        }

        None
    }

    fn iterate(&mut self) {}

    // Debug
    fn slots(&self) -> &[Slot] {
        &self.slots
    }

    fn name(&self) -> &'static str {
        self.name
    }
}
