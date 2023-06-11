use graphics::context::Context;
use graphics::Transformed;
use opengl_graphics::GlGraphics;

use crate::coordinate;
use crate::types;

pub trait Sign {
    fn color(&self) -> types::Color;
    fn size(&self) -> coordinate::Size;

    fn render(&self, context: &Context, gl: &mut GlGraphics, slot_size: coordinate::Size) {
        let size = self.size();

        graphics::Rectangle::new(self.color()).draw(
            [0.0, 0.0, size.width, size.height],
            &context.draw_state,
            context
                .transform
                .scale(size.width / slot_size.width, size.height / slot_size.height),
            gl,
        );
    }
}
