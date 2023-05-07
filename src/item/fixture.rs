use std::fmt::{Debug, Error, Formatter};

use graphics::context::Context;
use opengl_graphics::GlGraphics;

use crate::resource::Resource;
use crate::slot::Slot;
use crate::tile::Tile;
use crate::types;

pub trait Fixture
where
    Self: Iterator,
{
    fn direction(&self) -> &types::Direction;
    fn set_direction(&mut self, direction: types::Direction);

    fn rotate(&mut self) {
        self.set_direction(self.direction().next());
    }

    fn render(&self, gl: &mut GlGraphics, context: &Context);

    fn on_click(&mut self);

    fn before_update(&mut self, dt: f64) {
        self.set_cooling_time(self.cooling_time() + dt);
    }

    fn after_update(&mut self) {
        if self.operatable() {
            self.set_cooling_time(0.0);
        }
    }

    fn update(&mut self, dt: f64) {
        self.before_update(dt);
        if self.operatable() {
            self.iterate();
        }
        self.after_update();
    }

    fn operatable(&self) -> bool {
        self.cooling_time() > 0.5
    }

    fn set_cooling_time(&mut self, dt: f64);
    fn cooling_time(&self) -> f64;

    fn effect_range(&self) -> Option<types::GridSize>;
    fn affect(&mut self, target: &mut Tile, direction: &types::Direction);
    fn acceptable(&self) -> bool;
    fn push(&mut self, resource: Option<Resource>) -> Result<(), &'static str>;
    fn request(&mut self) -> Option<Resource>;

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

pub trait Iterator {
    fn iterate(&mut self);
}
