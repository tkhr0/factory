use graphics::context::Context;
use graphics::Transformed;
use opengl_graphics::GlGraphics;

use crate::item::Sign;
use crate::types;
use crate::QuickSlot as QuickSlotState;

pub struct QuickSlot {}

impl QuickSlot {
    const COLOR_UI_BORDER: types::Color = [0.5098, 0.5098, 0.5019, 1.0];
    const COLOR_UI_BODY: types::Color = [0.5098, 0.5098, 0.5019, 0.7];

    const SLOT_WIDTH: f64 = 50.0;
    const SLOT_HEIGHT: f64 = 50.0;

    const PADDING: f64 = 10.0;

    pub fn render(
        context: &Context,
        gl: &mut GlGraphics,
        hud_size: &types::Size,
        quick_slot: &QuickSlotState,
    ) {
        // QuickSlot frame
        let quick_slot_len = quick_slot.len() as f64;
        let quick_slot_size = (
            Self::PADDING + (Self::SLOT_WIDTH + Self::PADDING) * quick_slot_len,
            Self::SLOT_HEIGHT + Self::PADDING * 2.0,
        );
        let transform_quick_slot = context.transform.trans(
            (hud_size.width - quick_slot_size.0) / 2.0,
            hud_size.height - quick_slot_size.1 - 10.0,
        );
        graphics::Rectangle::new_border(Self::COLOR_UI_BORDER, 1.0)
            .color(Self::COLOR_UI_BODY)
            .draw(
                [0.0, 0.0, quick_slot_size.0, quick_slot_size.1],
                &context.draw_state,
                transform_quick_slot,
                gl,
            );

        // QuickSlot slots
        for (i, item) in quick_slot.builders().iter().enumerate() {
            let mut context = *context;
            context.transform = transform_quick_slot.trans(
                Self::PADDING + i as f64 * Self::SLOT_WIDTH + i as f64 * Self::PADDING,
                Self::PADDING,
            );
            graphics::Rectangle::new_border(Self::COLOR_UI_BORDER, 1.0)
                .color([1.0, 1.0, 1.0, 1.0])
                .draw(
                    [0.0, 0.0, Self::SLOT_WIDTH, Self::SLOT_HEIGHT],
                    &context.draw_state,
                    context.transform,
                    gl,
                );

            if let Some(item) = item {
                Sign::render(
                    item.build().as_ref(),
                    &context,
                    gl,
                    types::Size::new(Self::SLOT_WIDTH, Self::SLOT_HEIGHT),
                );
            }
        }
    }
}
