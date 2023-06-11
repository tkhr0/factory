use graphics::context::Context;
use graphics::Transformed;
use opengl_graphics::GlGraphics;
use piston::input::{ButtonArgs, ButtonState};
use piston::ResizeArgs;

use crate::coordinate;
use crate::item::Fixture;
use crate::EventHandleState;
use crate::PlayerState;
use crate::TileState;

mod inventory;
use inventory::Inventory;

mod quick_slot;
use quick_slot::QuickSlot;

pub struct Hud {
    size: coordinate::Size,
    inventory: Inventory,
    quick_slot: QuickSlot,
}

impl Hud {
    pub fn new(size: coordinate::Size, quick_slot_len: usize) -> Self {
        let inventory = Inventory::new(size);
        let quick_slot = QuickSlot::new(size, quick_slot_len);
        Self {
            size,
            inventory,
            quick_slot,
        }
    }

    pub fn render(
        &self,
        context: &Context,
        gl: &mut GlGraphics,
        player_state: &PlayerState,
        mouse_pos: &coordinate::Point,
        tile_state: TileState,
    ) {
        let holding_item = player_state.holding_item();

        self.quick_slot
            .render(context, gl, player_state.quick_slot(), holding_item);

        println!("{:?}", tile_state);

        if player_state.shown_inventory() {
            self.inventory.render(player_state.inventory(), gl, context);
        }

        // machine preview
        if let Some(variant) = holding_item {
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
        mouse_pos: &coordinate::Point,
        mut event_handle_state: EventHandleState,
        player_state: &mut PlayerState,
    ) -> EventHandleState {
        if args.state == ButtonState::Press {
            if let Some(index) = self.quick_slot.clicked(mouse_pos) {
                if let Some(item) = player_state.quick_slot()[index] {
                    player_state.hold_item(item)
                }
                event_handle_state.consume();
            }
        }

        event_handle_state
    }

    pub fn resize(&mut self, args: &ResizeArgs) {
        self.size = coordinate::Size::new(args.window_size[0], args.window_size[1]);
        self.inventory.resize(self.size);
        self.quick_slot.resize(self.size);
    }
}
