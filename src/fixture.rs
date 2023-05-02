use std::fmt::{Debug, Error, Formatter};

use graphics::context::Context;
use opengl_graphics::GlGraphics;

use crate::resource::Resource;
use crate::slot::Slot;
use crate::tile::Tile;
use crate::types;
use crate::GridSize;

pub trait Fixture
where
    Self: Iterator,
{
    fn direction(&self) -> &types::Direction;
    fn render(&self, gl: &mut GlGraphics, context: &Context);
    fn rotate(&mut self);
    fn on_click(&mut self);
    fn update(&mut self, dt: f64);

    fn before_update(&mut self, dt: f64) {
        self.set_cooling_time(self.cooling_time() + dt);
    }

    fn after_update(&mut self) {
        if self.operatable() {
            self.set_cooling_time(0.0);
        }
    }

    fn operatable(&self) -> bool {
        self.cooling_time() > 0.5
    }

    fn set_cooling_time(&mut self, dt: f64);
    fn cooling_time(&self) -> f64;

    fn effect_range(&self) -> Option<GridSize>;
    fn affect(&mut self, target: &mut Tile, direction: &types::Direction);
    fn acceptable(&self) -> bool;
    fn push(&mut self, resource: Option<Resource>) -> Result<(), &'static str>;

    // Debug
    fn slots(&self) -> &Vec<Slot>;
    fn name(&self) -> &'static str;
}

impl Debug for Box<dyn Fixture> {
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
