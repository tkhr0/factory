use opengl_graphics::GlGraphics;
use piston::input::{ButtonArgs, RenderArgs, UpdateArgs};

use crate::field::Field;
use crate::hud::Hud;
use crate::player_state::PlayerState;
use crate::types::Point;
use crate::ItemBuilders;

pub struct App<'a> {
    gl: GlGraphics, // OpenGL drawing backend.
    field: Field,
    player_state: PlayerState<'a>,
}

impl<'a> App<'a> {
    pub fn new<'b>(gl: GlGraphics) -> App<'b> {
        App {
            gl,
            field: Field::new(),
            player_state: Default::default(),
        }
    }

    pub fn initialize(&mut self, builders: &'a ItemBuilders) {
        self.field.initialize();
        self.player_state.initialize(builders);
    }

    pub fn render(&mut self, args: &RenderArgs) {
        const BACKGROUND: [f32; 4] = [252.0, 249.0, 230.0, 1.0];

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            graphics::clear(BACKGROUND, gl);
            self.field.render(gl, &c);
            Hud::render(&c, gl, &self.player_state);
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        self.field.update(args.dt);
    }

    pub fn button(&mut self, args: &ButtonArgs, mouse_pos: &Point) {
        self.field.on_click(args, mouse_pos);
    }
}
