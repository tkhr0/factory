use opengl_graphics::GlGraphics;
use piston::input::{ButtonArgs, RenderArgs, UpdateArgs};
use piston::ResizeArgs;

use crate::types;
use crate::EventHandleState;
use crate::Field;
use crate::Hud;
use crate::PlayerState;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    field: Field,
    hud: Hud,
}

impl App {
    pub fn new(window_size: types::Size, gl: GlGraphics, quick_slot_len: usize) -> App {
        App {
            gl,
            field: Field::new(),
            hud: Hud::new(window_size, quick_slot_len),
        }
    }

    pub fn initialize(&mut self) {
        self.field.initialize();
    }

    pub fn render(
        &mut self,
        args: &RenderArgs,
        player_state: &PlayerState,
        mouse_pos: &types::Point,
    ) {
        const BACKGROUND: [f32; 4] = [252.0, 249.0, 230.0, 1.0];

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            graphics::clear(BACKGROUND, gl);
            self.field.render(gl, &c);
            self.hud.render(&c, gl, player_state, mouse_pos);
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        self.field.update(args.dt);
    }

    pub fn button(
        &mut self,
        args: &ButtonArgs,
        mouse_pos: &types::Point,
        player_state: &mut PlayerState,
    ) {
        let mut state: EventHandleState = Default::default();
        state = self.hud.click(args, mouse_pos, state, player_state.quick_slot_mut());

        if !state.consumed() {
            self.field.on_click(
                args,
                mouse_pos,
                player_state.quick_slot().selected_item().map(|v| v.as_machine().unwrap()),
            );
        }
    }

    pub fn resize(&mut self, args: &ResizeArgs) {
        self.hud.resize(args);
    }
}
