use graphics::context::Context;
use graphics::Transformed;
use opengl_graphics::GlGraphics;

use crate::item::{ItemFactory, Sign};
use crate::types;
use crate::QuickSlot as QuickSlotState;

pub struct QuickSlot {
    hud_size: types::Size,
    slot_len: usize,
}

impl QuickSlot {
    const COLOR_UI_BORDER: types::Color = [0.5098, 0.5098, 0.5019, 1.0];
    const COLOR_UI_BODY: types::Color = [0.5098, 0.5098, 0.5019, 0.7];
    const COLOR_UI_SELECTED: types::Color = [0.9215, 0.3529, 0.3215, 0.5];

    const SLOT_WIDTH: f64 = 50.0;
    const SLOT_HEIGHT: f64 = 50.0;

    const PADDING: f64 = 10.0;

    pub fn new(hud_size: types::Size, slot_len: usize) -> Self {
        Self { hud_size, slot_len }
    }

    pub fn render(&self, context: &Context, gl: &mut GlGraphics, quick_slot: &QuickSlotState) {
        // QuickSlot frame
        let size = self.size();
        let origin = self.origin();
        let transform_quick_slot = context.transform.trans(origin.x, origin.y);
        graphics::Rectangle::new_border(Self::COLOR_UI_BORDER, 1.0)
            .color(Self::COLOR_UI_BODY)
            .draw(
                [0.0, 0.0, size.width, size.height],
                &context.draw_state,
                transform_quick_slot,
                gl,
            );

        // QuickSlot slots
        let selected = quick_slot.selected();
        for (i, item) in quick_slot.items().iter().enumerate() {
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
                    ItemFactory::build(*item).as_ref(),
                    &context,
                    gl,
                    types::Size::new(Self::SLOT_WIDTH, Self::SLOT_HEIGHT),
                );
            }

            if i == selected {
                graphics::Rectangle::new_border(Self::COLOR_UI_SELECTED, 1.0).draw(
                    [0.0, 0.0, Self::SLOT_WIDTH, Self::SLOT_HEIGHT],
                    &context.draw_state,
                    context.transform,
                    gl,
                );
            }
        }
    }

    pub fn resize(&mut self, hud_size: types::Size) {
        self.hud_size = hud_size;
    }

    pub fn clicked(&self, pos: &types::Point) -> Option<usize> {
        let origin = &self.origin();
        let size = &self.size();

        // Quick Slot is not clicked
        let padding = types::Size::new(Self::PADDING, Self::PADDING);
        if !((origin + &padding) <= *pos && *pos < (origin + &(size - &padding))) {
            return None;
        }

        let local_pos = pos - origin;

        // TODO: padding is not considered.
        //       this is valid if click on padding.
        Some((local_pos.x / (Self::SLOT_WIDTH + Self::PADDING)) as usize)
    }

    fn size(&self) -> types::Size {
        types::Size::new(
            Self::PADDING + (Self::SLOT_WIDTH + Self::PADDING) * self.slot_len as f64,
            Self::SLOT_HEIGHT + Self::PADDING * 2.0,
        )
    }

    fn origin(&self) -> types::Point {
        let size = self.size();

        types::Point::new(
            (self.hud_size.width - size.width) / 2.0,
            self.hud_size.height - size.height - 10.0,
        )
    }
}

#[cfg(test)]
mod quick_slot_test {
    mod clicked {
        use crate::hud::quick_slot::QuickSlot;
        use crate::types::{Point, Size};

        #[test]
        fn test_range_out_of_y_axis() {
            let clicked =
                QuickSlot::new(Size::new(500.0, 500.0), 1).clicked(&Point::new(235.0, 1.0));

            assert_eq!(clicked, None);
        }

        #[test]
        fn test_range_out_of_x_axis() {
            let clicked =
                QuickSlot::new(Size::new(500.0, 500.0), 1).clicked(&Point::new(1.0, 420.0));

            assert_eq!(clicked, None);
        }
    }
}
