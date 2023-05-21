use graphics::context::Context;
use graphics::Transformed;
use opengl_graphics::GlGraphics;
use piston::input::{ButtonArgs, ButtonState};
use piston::ResizeArgs;

use crate::item::Fixture;
use crate::types;
use crate::EventHandleState;
use crate::PlayerState;
use crate::QuickSlot as QuickSlotState;

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

    pub fn render(
        &self,
        context: &Context,
        gl: &mut GlGraphics,
        player_state: &PlayerState,
        mouse_pos: &types::Point,
    ) {
        self.quick_slot
            .render(context, gl, player_state.quick_slot());

        if player_state.shown_inventory() {
            println!("shown inventory");
        }

        // machine preview
        if let Some(variant) = player_state.quick_slot().selected_item() {
            if let Some(machine) = variant.as_machine() {
                let mut context = *context;
                context.transform = context.transform.trans(mouse_pos.x, mouse_pos.y);

                let machine: Box<dyn Fixture> = machine;

                machine.render(gl, &context);
            }
        }
    }

    pub fn click(
        &mut self,
        args: &ButtonArgs,
        mouse_pos: &types::Point,
        mut event_handle_state: EventHandleState,
        quick_slot_state: &mut QuickSlotState,
    ) -> EventHandleState {
        if args.state == ButtonState::Press {
            if let Some(index) = self.quick_slot.clicked(mouse_pos) {
                quick_slot_state.select(index);
                event_handle_state.consume();
            }
        }

        event_handle_state
    }

    pub fn resize(&mut self, args: &ResizeArgs) {
        self.size = types::Size::new(args.window_size[0], args.window_size[1]);
        self.quick_slot.resize(self.size);
    }
}
