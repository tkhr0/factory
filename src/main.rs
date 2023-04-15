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
}

type Point = [f64; 2];

impl App {
    fn render(&mut self, args: &RenderArgs) {

        const BACKGROUND: [f32; 4] = [252.0, 249.0, 230.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let rotation = self.rotation;


        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BACKGROUND, gl);

            for rect in self.rectangles.iter() {
                let [x, y, _, _] = rect;
                let transform = c
                    .transform
                    .trans(x.to_owned(), y.to_owned())
                    .rot_rad(rotation)
                    .trans(-25.0, -25.0);

                // Draw a box rotating around the middle of the screen.
                rectangle(RED, rect.to_owned(), transform, gl);
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
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create a Glutin window.
    let mut window: Window = WindowSettings::new("spinning-square", [200, 200])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        rotation: 0.0,
        rectangles: Vec::new(),
    };

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
