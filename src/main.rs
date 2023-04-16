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
    rotation: f64,  // Rotation for the square.
    rectangles: Vec<graphics::types::Rectangle>,
    machines: Vec<Machine>,
}

type Point = [f64; 2];
type Size = [f64; 2];

impl App {
    fn render(&mut self, args: &RenderArgs) {
        const BACKGROUND: [f32; 4] = [252.0, 249.0, 230.0, 1.0];

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BACKGROUND, gl);

            for machine in self.machines.iter() {
                machine.render(gl, &c);
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt;
    }

    pub fn button(&mut self, button_args: &ButtonArgs, mouse_args: &Point) {
        if button_args.state == ButtonState::Press {
            self.rectangles
                .push(rectangle::square(mouse_args[0], mouse_args[1], 50.0));
        }
    }

    pub fn add_machine(&mut self, machine: Machine) {
        self.machines.push(machine);
    }
}

pub struct Machine {
    position: Point,
}

impl Machine {
    pub fn new(position: Point) -> Machine {
        Machine { position }
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
        rotation: 0.0,
        rectangles: Vec::new(),
        machines: Vec::new(),
    };

    app.add_machine(Machine::new([50.0, 50.0]));
    app.add_machine(Machine::new([150.0, 150.0]));

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
