use std::fmt::{Debug, Error, Formatter};

use graphics::context::Context;
use opengl_graphics::GlGraphics;

use crate::types;

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
}

impl Debug for Box<dyn Fixture> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.debug_struct("Fixture").finish()
    }
}

pub trait Iterator {
    fn iterate(&mut self);
}

// pub trait Passable where Self: Pickable {
//     fn pass(&mut self, target: Option<&mut Box<dyn Pushable>>) -> Result<(), &'static str> {
//         if let Some(target) = target {
//             if target.acceptable() {
//                 return target.push(self.pick());
//             }
//         }
//         Ok(())
//     }
// }
//
// pub trait Pushable {
//     fn push(&mut self, resource: Option<Resource>) -> Result<(), &'static str>;
//     fn acceptable(&self) -> bool;
// }
//
// pub trait Pickable {
//     fn pick(&mut self) -> Option<Resource>;
// }
//
// impl Debug for Box<dyn Pushable> {
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
//         f.debug_struct("Pushable").finish()
//     }
// }
