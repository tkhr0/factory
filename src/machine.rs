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

    pub fn render(&self, gl: &mut GlGraphics, context: &Context) {
        const BODY: [f32; 4] = [0.749, 0.741, 0.329, 1.0];
        const RESOURCE: [f32; 4] = [0.9803, 0.9803, 0.9607, 1.0];

        let size = self.size();

        graphics::Rectangle::new(BODY).draw(
            [0.0, 0.0, size.width, size.height],
            &context.draw_state,
            context.transform,
            gl,
        );

        for (i, resource) in self.container.iter().enumerate() {
            if resource.is_some() {
                graphics::ellipse(
                    RESOURCE,
                    [20.0, ((i as f64) * 10.0) + 5.0, 10.0, 10.0],
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
        self.cooling_time() > 0.5
    }
}
