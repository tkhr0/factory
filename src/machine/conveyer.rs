use graphics::context::Context;
use graphics::Transformed;
use opengl_graphics::GlGraphics;

use crate::container::Container;
use crate::machine::{Clickable, MachineCore};
use crate::resource::Resource;
use crate::types;

#[derive(Debug)]
pub struct Conveyer {
    #[allow(dead_code)]
    name: &'static str,
    container: Container,
    cooling_time: f64,
    direction: types::Direction,
}

impl Conveyer {
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            container: Container::new(),
            cooling_time: 0.0,
            direction: Default::default(),
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

    pub fn rotate(&mut self) {
        self.set_direction(self.direction().next());
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

    pub fn direction(&self) -> &types::Direction {
        &self.direction
    }

    fn set_direction(&mut self, direction: types::Direction) {
        self.direction = direction;
    }

    fn angle(&self) -> types::Radian {
        self.direction().angle()
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

        for (i, resource) in self.container.iter().enumerate() {
            if resource.is_some() {
                graphics::ellipse(
                    RESOURCE,
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
}

impl MachineCore<Resource> for Conveyer {
    fn set_cooling_time(&mut self, dt: f64) {
        self.cooling_time = dt;
    }

    fn cooling_time(&self) -> f64 {
        self.cooling_time
    }

    fn main(&mut self, target: Option<&mut Self>) {
        if let Some(target) = target {
            if target.container.acceptable() {
                target.push(self.pick()).unwrap();
            }
        }

        self.container.update();
    }
}

impl Clickable for Conveyer {
    fn on_click(&mut self) {
        self.load();
    }
}

pub struct ConveyerBuilder {
    name: &'static str,
    direction: Option<types::Direction>,
}

impl ConveyerBuilder {
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            direction: Default::default(),
        }
    }

    pub fn set_direction(&mut self, direction: types::Direction) -> &mut Self {
        self.direction = Some(direction);
        self
    }

    pub fn build(&mut self) -> Conveyer {
        let direction = self.direction.take().unwrap_or_default();
        Conveyer {
            name: self.name,
            container: Container::new(),
            cooling_time: 0.0,
            direction,
        }
    }
}
