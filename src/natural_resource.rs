use graphics::context::Context;
use opengl_graphics::GlGraphics;

use crate::types;

pub trait NaturalResource {
    fn color(&self) -> [f32; 4];

    fn render(&self, gl: &mut GlGraphics, context: &Context, size: &types::Size) {
        graphics::Rectangle::new(self.color()).draw(
            [0.0, 0.0, size.width, size.height],
            &context.draw_state,
            context.transform,
            gl,
        );
    }
}

impl std::fmt::Debug for Box<dyn NaturalResource> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NaturalResource").finish()
    }
}
