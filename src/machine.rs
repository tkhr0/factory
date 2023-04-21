use graphics::context::Context;
use opengl_graphics::GlGraphics;

use crate::container::Container;
use crate::resource::Resource;
use crate::types;

#[derive(Debug)]
pub struct Machine {
    #[allow(dead_code)]
    name: &'static str,
    container: Container,
    cooling_time: f64,
}

impl Machine {
    pub fn new(name: &'static str) -> Machine {
        Machine {
            name,
            container: Container::new(),
            cooling_time: 0.0,
        }
    }

    pub fn load(&mut self) {
        self.container.load();
    }

    fn pick(&mut self) -> Option<Resource> {
        self.container.pick()
    }

    fn push(&mut self, resource: Option<Resource>) -> Result<(), &'static str> {
        self.container.push(resource)
    }

    fn width(&self) -> f64 {
        50.0
    }

    fn height(&self) -> f64 {
        50.0
    }

    fn size(&self) -> types::Size {
        types::Size::new(self.width(), self.height())
    }

    fn top_left(&self) -> types::Point {
        types::Point::new(0.0, 0.0)
    }

    fn top_right(&self) -> types::Point {
        types::Point::new(self.width(), 0.0)
    }

    fn bottom_left(&self) -> types::Point {
        types::Point::new(0.0, self.height())
    }

    fn bottom_right(&self) -> types::Point {
        types::Point::new(self.width(), self.height())
    }

    pub fn render(&self, gl: &mut GlGraphics, context: &Context) {
        const BODY: [f32; 4] = [255.0, 186.0, 3.0, 1.0];
        const OUTLINE: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        let size = self.size();

        graphics::rectangle(
            BODY,
            [0.0, 0.0, size.width, size.height],
            context.transform,
            gl,
        );
        graphics::line(
            OUTLINE,
            1.0,
            [
                self.top_left().x,
                self.top_left().y,
                self.top_right().x,
                self.top_right().y,
            ],
            context.transform,
            gl,
        );
        graphics::line(
            OUTLINE,
            1.0,
            [
                self.top_right().x,
                self.top_right().y,
                self.bottom_right().x,
                self.bottom_right().y,
            ],
            context.transform,
            gl,
        );
        graphics::line(
            OUTLINE,
            1.0,
            [
                self.bottom_right().x,
                self.bottom_right().y,
                self.bottom_left().x,
                self.bottom_left().y,
            ],
            context.transform,
            gl,
        );
        graphics::line(
            OUTLINE,
            1.0,
            [
                self.bottom_left().x,
                self.bottom_left().y,
                self.top_left().x,
                self.top_left().y,
            ],
            context.transform,
            gl,
        );

        for (i, resource) in self.container.iter().enumerate() {
            if resource.is_some() {
                graphics::ellipse(
                    OUTLINE,
                    [((i as f64) * 10.0), 0.0, 10.0, 10.0],
                    context.transform,
                    gl,
                );
            }
        }
    }
}

impl MachineCore<Resource> for Machine {
    fn set_cooling_time(&mut self, dt: f64) {
        self.cooling_time = dt;
    }

    fn cooling_time(&self) -> f64 {
        self.cooling_time
    }

    fn main(&mut self, neighbor: Option<&mut Self>) {
        self.container.update();

        if let Some(neighbor) = neighbor {
            if self.container.acceptable() {
                self.push(neighbor.pick()).unwrap();
            }
        }
    }
}

pub trait Clickable {
    fn on_click(&mut self);
}

impl Clickable for Machine {
    fn on_click(&mut self) {
        self.load();
    }
}

pub trait MachineCore<S> {
    fn update(&mut self, dt: f64, neighbor: Option<&mut Self>) {
        self.before_update(dt);
        if self.operatable() {
            self.main(neighbor);
        }
        self.after_update();
    }

    fn main(&mut self, neighbor: Option<&mut Self>);

    fn before_update(&mut self, dt: f64) {
        self.set_cooling_time(self.cooling_time() + dt);
    }

    fn after_update(&mut self) {
        if self.operatable() {
            self.set_cooling_time(0.0);
        }
    }

    fn set_cooling_time(&mut self, dt: f64);
    fn cooling_time(&self) -> f64;

    fn operatable(&self) -> bool {
        self.cooling_time() > 1.0
    }
}
