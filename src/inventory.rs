use graphics::context::Context;
use graphics::Transformed;
use opengl_graphics::GlGraphics;

use crate::item::{MaterialVariant, Sign};
use crate::types;

const COLUMNS: usize = 4;
const ROWS: usize = 4;
const SLOTS: usize = COLUMNS * ROWS;

pub struct Inventory {
    items: [InventorySlot; SLOTS],
}

impl Inventory {
    const COLOR_UI_BORDER: types::Color = [0.5098, 0.5098, 0.5019, 1.0];
    const COLOR_UI_BODY: types::Color = [0.5098, 0.5098, 0.5019, 0.7];

    pub fn render(&self, gl: &mut GlGraphics, context: &Context) {
        let size = self.size();

        graphics::Rectangle::new_border(Self::COLOR_UI_BORDER, 1.0)
            .color(Self::COLOR_UI_BODY)
            .draw(
                [0.0, 0.0, size.width, size.height],
                &context.draw_state,
                context.transform,
                gl,
            );

        for (i, slot) in self.items.iter().enumerate() {
            let mut context = *context;
            context.transform = context
                .transform
                .trans((i % COLUMNS) as f64 * WIDTH, (i / COLUMNS) as f64 * HEIGHT);
            slot.render(gl, &context);
        }
    }

    fn size(&self) -> types::Size {
        types::Size::new((ROWS as f64) * WIDTH + 1.0, (COLUMNS as f64) * HEIGHT + 1.0)
    }
}

impl Default for Inventory {
    fn default() -> Self {
        Self {
            items: std::array::from_fn(|i| match i {
                // sample
                0 => InventorySlot::new(MaterialVariant::Coal),
                1 => InventorySlot::new(MaterialVariant::Conveyer),
                _ => Default::default(),
            }),
        }
    }
}

const WIDTH: f64 = 50.0;
const HEIGHT: f64 = 50.0;
const SIZE: types::Size = types::Size::new(WIDTH, HEIGHT);

#[derive(Default)]
struct InventorySlot {
    variant: Option<MaterialVariant>,
    // count: u32,
}

impl InventorySlot {
    pub fn new(variant: MaterialVariant) -> Self {
        Self {
            variant: Some(variant),
        }
    }

    pub fn render(&self, gl: &mut GlGraphics, context: &Context) {
        if let Some(variant) = &self.variant {
            let sign: Box<dyn Sign> = variant.as_material();
            sign.render(context, gl, SIZE);
        }
    }
}
