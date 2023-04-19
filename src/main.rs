#![feature(get_many_mut)]
#![feature(associated_type_defaults)]

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use graphics::*;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{
    ButtonArgs, ButtonEvent, ButtonState, MouseCursorEvent, RenderArgs, RenderEvent, UpdateArgs,
    UpdateEvent,
};
use piston::window::WindowSettings;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    field: Field,
}

type Point = [f64; 2];
type Size = [f64; 2];

impl App {
    fn initialize(&mut self) {
        self.field.initialize();
    }

    fn render(&mut self, args: &RenderArgs) {
        const BACKGROUND: [f32; 4] = [252.0, 249.0, 230.0, 1.0];

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BACKGROUND, gl);
            self.field.render(gl, &c);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.field.update(args.dt);
    }

    pub fn button(&mut self, args: &ButtonArgs, mouse_pos: &Point) {
        self.field.on_click(args, mouse_pos);
    }
}

#[derive(Debug)]
pub struct Field {
    size: Size,
    machines: Vec<Machine>,
}

impl Field {
    pub fn new() -> Field {
        Field {
            size: [500.0, 500.0],
            machines: Vec::new(),
        }
    }

    pub fn initialize(&mut self) {
        self.add_machine(Machine::new("A", [50.0, 50.0], 0.0));
        self.add_machine(Machine::new("B", [150.0, 150.0], 0.3));
        self.add_machine(Machine::new("C", [220.0, 150.0], 0.8));
    }

    pub fn size(&self) -> Size {
        self.size
    }

    pub fn add_machine(&mut self, machine: Machine) {
        self.machines.push(machine);
    }

    pub fn render(&self, gl: &mut GlGraphics, context: &Context) {
        for machine in self.machines.iter() {
            machine.render(gl, context);
        }
    }

    pub fn update(&mut self, dt: f64) {
        for i in 0..self.machines.len() {
            if i > 0 {
                if let Ok([prev, current]) = self.machines.get_many_mut([i - 1, i]) {
                    current.update(dt, prev);
                }
            }
        }
        println!("{:?}", self.machines);
    }

    pub fn on_click(&mut self, args: &ButtonArgs, mouse_pos: &Point) {
        if args.state == ButtonState::Press
            && args.button == piston::Button::Mouse(piston::MouseButton::Left)
        {
            for machine in self.machines.iter_mut() {
                if machine.collided(mouse_pos) {
                    machine.on_click();
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct Resource {}

trait Clickable {
    fn on_click(&mut self);
}

trait Conveyor<S> {
    type Container = Vec<Option<S>>;

    fn update(&mut self, dt: f64, prev: &mut Self) {
        self.before_update(dt);
        if self.operatable() {
            self.main(prev);
        }
        self.after_update();
    }

    fn main(&mut self, prev: &mut Self);

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

#[derive(Debug)]
pub struct Machine {
    name: &'static str,
    position: Point,
    container: Vec<Option<Resource>>,
    cooling_time: f64,
}

impl Machine {
    pub fn new(name: &'static str, position: Point, cooling_time: f64) -> Machine {
        Machine {
            name,
            position,
            container: vec![None, None, None, None],
            cooling_time,
        }
    }

    pub fn name(&self) -> &str {
        self.name
    }

    pub fn load(&mut self) {
        self.container.push(Some(Resource {}));
    }

    pub fn position(&self) -> Point {
        self.position
    }

    fn width(&self) -> f64 {
        50.0
    }

    fn height(&self) -> f64 {
        50.0
    }

    pub fn size(&self) -> Size {
        [self.width(), self.height()]
    }

    fn top_left(&self) -> Point {
        [self.position[0], self.position[1]]
    }

    fn top_right(&self) -> Point {
        [self.position[0] + self.width(), self.position[1]]
    }

    fn bottom_left(&self) -> Point {
        [self.position[0], self.position[1] + self.height()]
    }

    fn bottom_right(&self) -> Point {
        [
            self.position[0] + self.width(),
            self.position[1] + self.height(),
        ]
    }

    pub fn collided(&self, point: &Point) -> bool {
        let x = point[0];
        let y = point[1];
        let top_left = self.top_left();
        let top = top_left[1];
        let left = top_left[0];
        let bottom_right = self.bottom_right();
        let right = bottom_right[0];
        let bottom = bottom_right[1];
        left <= x && x <= right && top <= y && y <= bottom
    }

    pub fn render(&self, gl: &mut GlGraphics, context: &Context) {
        const BODY: [f32; 4] = [255.0, 186.0, 3.0, 1.0];
        const OUTLINE: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        let position = self.position();
        let size = self.size();

        rectangle(
            BODY,
            [position[0], position[1], size[0], size[1]],
            context.transform,
            gl,
        );
        line(
            OUTLINE,
            1.0,
            [
                self.top_left()[0],
                self.top_left()[1],
                self.top_right()[0],
                self.top_right()[1],
            ],
            context.transform,
            gl,
        );
        line(
            OUTLINE,
            1.0,
            [
                self.top_right()[0],
                self.top_right()[1],
                self.bottom_right()[0],
                self.bottom_right()[1],
            ],
            context.transform,
            gl,
        );
        line(
            OUTLINE,
            1.0,
            [
                self.bottom_right()[0],
                self.bottom_right()[1],
                self.bottom_left()[0],
                self.bottom_left()[1],
            ],
            context.transform,
            gl,
        );
        line(
            OUTLINE,
            1.0,
            [
                self.bottom_left()[0],
                self.bottom_left()[1],
                self.top_left()[0],
                self.top_left()[1],
            ],
            context.transform,
            gl,
        );

        for i in 0..self.container.len() {
            ellipse(
                OUTLINE,
                [position[0] + (i as f64) * 10.0, position[1], 10.0, 10.0],
                context.transform,
                gl,
            );
        }
    }

    pub fn pull_resource(&mut self, prev_machine: &mut Machine) {
        if !prev_machine.container.is_empty() {
            self.container.push(prev_machine.container.pop().unwrap());
        }
    }
}

impl Conveyor<Resource> for Machine {
    fn set_cooling_time(&mut self, dt: f64) {
        self.cooling_time = dt;
    }

    fn cooling_time(&self) -> f64 {
        self.cooling_time
    }

    fn main(&mut self, prev: &mut Self) {
        self.pull_resource(prev);
    }
}

impl Clickable for Machine {
    fn on_click(&mut self) {
        self.load();
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create a Glutin window.
    let mut window: Window = WindowSettings::new("spinning-square", [500, 500])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        field: Field::new(),
    };

    app.initialize();

    let mut mouse_pos = [0.0, 0.0];
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(mouse_args) = e.mouse_cursor_args() {
            mouse_pos = mouse_args;
        }

        if let Some(args) = e.button_args() {
            println!("Button: {:?}", args);
            app.button(&args, &mouse_pos);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}