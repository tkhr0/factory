use graphics::context::Context;
use graphics::Transformed;
use opengl_graphics::GlGraphics;

use crate::resource::Resource;
use crate::tile::Tile;
use crate::types;
use crate::GridSize;
use crate::Slot;
use crate::{Fixture, Iterator};

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

    fn angle(&self) -> types::Radian {
        self.direction().angle()
    }

    fn load(&mut self) {
        let _ = self.push(Some(Resource::default()));
    }
}

impl<const N: usize> Fixture for Container<N> {
    fn direction(&self) -> &types::Direction {
        &self.direction
    }

    fn set_direction(&mut self, direction: types::Direction) {
        self.direction = direction;
    }

    fn render(&self, gl: &mut GlGraphics, context: &Context) {
        const BODY: [f32; 4] = [0.8117, 0.5019, 0.0078, 1.0];
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

    fn effect_range(&self) -> Option<GridSize> {
        None
    }
    fn affect(&mut self, _target: &mut Tile, _direction: &types::Direction) {}

    fn acceptable(&self) -> bool {
        self.slots.iter().any(|slot| slot.is_empty())
    }

    fn push(&mut self, resource: Option<Resource>) -> Result<(), &'static str> {
        for slot in self.slots.iter_mut() {
            if slot.is_empty() {
                return slot.push(resource);
            }
        }

        Err("No empty slot")
    }

    // Debug
    fn slots(&self) -> &[Slot] {
        &self.slots
    }

    fn name(&self) -> &'static str {
        self.name
    }
}

impl<const N: usize> Iterator for Container<N> {
    fn iterate(&mut self) {}
}

pub struct ContainerBuilder {
    name: &'static str,
    direction: types::Direction,
}

impl ContainerBuilder {
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            direction: Default::default(),
        }
    }

    pub fn build(self) -> Box<dyn Fixture> {
        Box::new(Container::<16> {
            name: self.name,
            slots: core::array::from_fn(|_| Slot::default()),
            direction: self.direction,
        })
    }
}
