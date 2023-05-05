use opengl_graphics::GlGraphics;
use piston::input::{ButtonArgs, RenderArgs, UpdateArgs};
use piston::ResizeArgs;

use crate::field::Field;
use crate::hud::Hud;
use crate::player_state::PlayerState;
use crate::types;
use crate::ItemBuilders;

pub struct App<'a> {
    gl: GlGraphics, // OpenGL drawing backend.
    field: Field,
    player_state: PlayerState<'a>,
    hud: Hud,
}

impl<'a> App<'a> {
    pub fn new<'b>(window_size: types::Size, gl: GlGraphics) -> App<'b> {
        let player_state: PlayerState = Default::default();
        let quick_slot_len = player_state.quick_slot().len();

        App {
            gl,
            field: Field::new(),
            player_state,
            hud: Hud::new(window_size, quick_slot_len),
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
            self.hud.render(&c, gl, &self.player_state);
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        self.field.update(args.dt);
    }

    pub fn button(&mut self, args: &ButtonArgs, mouse_pos: &types::Point) {
        self.hud.click(args, mouse_pos);
        self.field.on_click(args, mouse_pos);
    }

    pub fn resize(&mut self, args: &ResizeArgs) {
        self.hud.resize(args);
    }
}
