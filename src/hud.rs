use graphics::context::Context;
use opengl_graphics::GlGraphics;
use piston::ResizeArgs;

use crate::player_state::PlayerState;
use crate::types;

mod quick_slot;
use quick_slot::QuickSlot;

pub struct Hud {
    size: types::Size,
}

impl Hud {
    pub fn new(size: types::Size) -> Self {
        Self { size }
    }

    pub fn render(&self, context: &Context, gl: &mut GlGraphics, player_state: &PlayerState) {
        QuickSlot::render(context, gl, &self.size, player_state.quick_slot());
    }

    pub fn resize(&mut self, args: &ResizeArgs) {
        self.size = types::Size::new(args.window_size[0], args.window_size[1]);
    }
}
