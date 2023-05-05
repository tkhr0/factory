use graphics::context::Context;
use opengl_graphics::GlGraphics;
use piston::ResizeArgs;

use crate::player_state::PlayerState;
use crate::types;

mod quick_slot;
use quick_slot::QuickSlot;

pub struct Hud {
    size: types::Size,
    quick_slot: QuickSlot,
}

impl Hud {
    pub fn new(size: types::Size, quick_slot_len: usize) -> Self {
        let quick_slot = QuickSlot::new(size, quick_slot_len);
        Self { size, quick_slot }
    }

    pub fn render(&self, context: &Context, gl: &mut GlGraphics, player_state: &PlayerState) {
        self.quick_slot
            .render(context, gl, player_state.quick_slot());
    }

    pub fn resize(&mut self, args: &ResizeArgs) {
        self.size = types::Size::new(args.window_size[0], args.window_size[1]);
        self.quick_slot.resize(self.size);
    }
}
