use graphics::context::Context;
use graphics::Transformed;
use opengl_graphics::GlGraphics;

use crate::coordinate;
use crate::item::{MaterialVariant, Sign};
use crate::types;
use crate::Inventory as InventoryState;

pub struct Inventory {
    hud_size: coordinate::Size,
}

impl Inventory {
    const COLOR_UI_BORDER: types::Color = [0.5098, 0.5098, 0.5019, 1.0];
    const COLOR_UI_BODY: types::Color = [0.5098, 0.5098, 0.5019, 0.7];

    pub fn new(hud_size: coordinate::Size) -> Self {
        Self { hud_size }
    }

    pub fn render(&self, inventory: &InventoryState, gl: &mut GlGraphics, context: &Context) {
        let size = self.size();
        let origin = self.origin();
        let transform = context.transform.trans(origin.x, origin.y);
        graphics::Rectangle::new_border(Self::COLOR_UI_BORDER, 1.0)
            .color(Self::COLOR_UI_BODY)
            .draw(
                [0.0, 0.0, size.width, size.height],
                &context.draw_state,
                transform,
                gl,
            );

        for (i, slot) in inventory.items().iter().enumerate() {
            if let Some(variant) = slot.variant() {
                let mut context = *context;
                context.transform = transform.trans(
                    (i % InventoryState::COLUMNS) as f64 * WIDTH,
                    (i / InventoryState::COLUMNS) as f64 * HEIGHT,
                );
                InventorySlot::render(gl, &context, variant);
            }
        }
    }

    fn size(&self) -> coordinate::Size {
        coordinate::Size::new(
            (InventoryState::ROWS as f64) * WIDTH + 1.0,
            (InventoryState::COLUMNS as f64) * HEIGHT + 1.0,
        )
    }

    fn origin(&self) -> coordinate::Point {
        let size = self.size();

        coordinate::Point::new(
            (self.hud_size.width - size.width) / 2.0,
            (self.hud_size.height - size.height) / 2.0,
        )
    }

    pub fn resize(&mut self, hud_size: coordinate::Size) {
        self.hud_size = hud_size;
    }
}

const WIDTH: f64 = 50.0;
const HEIGHT: f64 = 50.0;
const SIZE: coordinate::Size = coordinate::Size::new(WIDTH, HEIGHT);

#[derive(Default)]
struct InventorySlot {}

impl InventorySlot {
    pub fn render(gl: &mut GlGraphics, context: &Context, variant: MaterialVariant) {
        let sign: Box<dyn Sign> = variant.as_material();
        sign.render(context, gl, SIZE);
    }
}
