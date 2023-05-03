use graphics::context::Context;
use graphics::Transformed;
use opengl_graphics::GlGraphics;

use crate::types;

pub trait Item {
    fn color(&self) -> types::Color;
    fn size(&self) -> types::Size;

    fn render(&self, context: &Context, gl: &mut GlGraphics, slot_size: types::Size) {
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
