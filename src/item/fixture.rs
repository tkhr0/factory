use std::fmt::{Debug, Error, Formatter};

use graphics::context::Context;
use opengl_graphics::GlGraphics;

use crate::item::ResourceObj;
use crate::types;
use crate::Slot;
use crate::Tile;

pub trait Fixture {
    fn direction(&self) -> &types::Direction;
    fn set_direction(&mut self, direction: types::Direction);

    fn rotate(&mut self) {
        self.set_direction(self.direction().next());
    }

    fn render(&self, gl: &mut GlGraphics, context: &Context);

    fn on_click(&mut self);

    fn before_update(&mut self, dt: f64) {
        let next = if self.operatable() {
            0.0
        } else {
            self.cooling_time() + dt
        };
        self.set_cooling_time(next);
    }

    fn update(&mut self, dt: f64) {
        self.before_update(dt);
        if self.operatable() {
            self.iterate();
        }
    }

    fn operatable(&self) -> bool {
        self.cooling_time() > 0.5
    }

    fn set_cooling_time(&mut self, dt: f64);
    fn cooling_time(&self) -> f64;

    fn effect_range(&self) -> Option<types::GridSize>;
    fn iterate(&mut self);

    fn affect(&mut self, target: &mut Tile, direction: &types::Direction);

    fn insertable(&self) -> bool;
    fn insert(&mut self, resource: ResourceObj) -> Result<(), &'static str>;

    // for conveyers
    fn pushable(&self) -> bool;
    fn push(&mut self, resource: Option<ResourceObj>) -> Result<(), &'static str>;

    fn request(&mut self) -> Option<ResourceObj>;

    // Debug
    fn slots(&self) -> &[Slot];
    fn name(&self) -> &'static str;
}

impl Debug for dyn Fixture {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.debug_struct("Fixture")
            .field("name", &self.name())
            .field("slots", &self.slots())
            .finish()
    }
}
