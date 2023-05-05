use graphics::context::Context;
use graphics::Transformed;
use opengl_graphics::GlGraphics;

use crate::player_state::PlayerState;
use crate::types;

pub struct Hud {}

const COLOR_UI_BORDER: types::Color = [0.5098, 0.5098, 0.5019, 1.0];
const COLOR_UI_BODY: types::Color = [0.5098, 0.5098, 0.5019, 0.7];

const SLOT_WIDTH: f64 = 50.0;
const SLOT_HEIGHT: f64 = 50.0;

const PADDING_QUICK_SLOT: f64 = 10.0;

impl Hud {
    pub fn render(context: &Context, gl: &mut GlGraphics, player_state: &PlayerState) {
        if let Some(viewport) = context.viewport {
            let [width, height] = viewport.window_size;

            // QuickSlot frame
            let quick_slot_len = player_state.quick_slot().len() as f64;
            let quick_slot_size = (
                PADDING_QUICK_SLOT + (SLOT_WIDTH + PADDING_QUICK_SLOT) * quick_slot_len,
                SLOT_HEIGHT + PADDING_QUICK_SLOT * 2.0,
            );
            let transform_quick_slot = context.transform.trans(
                (width - quick_slot_size.0) / 2.0,
                height - quick_slot_size.1 - 10.0,
            );
            graphics::Rectangle::new_border(COLOR_UI_BORDER, 1.0)
                .color(COLOR_UI_BODY)
                .draw(
                    [0.0, 0.0, quick_slot_size.0, quick_slot_size.1],
                    &context.draw_state,
                    transform_quick_slot,
                    gl,
                );

            // QuickSlot slots
            for (i, item) in player_state.quick_slot().items().iter().enumerate() {
                let mut context = *context;
                context.transform = transform_quick_slot.trans(
                    PADDING_QUICK_SLOT + i as f64 * SLOT_WIDTH + i as f64 * PADDING_QUICK_SLOT,
                    PADDING_QUICK_SLOT,
                );
                graphics::Rectangle::new_border(COLOR_UI_BORDER, 1.0)
                    .color([1.0, 1.0, 1.0, 1.0])
                    .draw(
                        [0.0, 0.0, SLOT_WIDTH, SLOT_HEIGHT],
                        &context.draw_state,
                        context.transform,
                        gl,
                    );

                if let Some(item) = item {
                    item.render(&context, gl, types::Size::new(SLOT_WIDTH, SLOT_HEIGHT));
                }
            }
        }
    }
}