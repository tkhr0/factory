pub mod coal;
pub use coal::Coal;

pub mod iron_ore;
pub use iron_ore::IronOre;

use graphics::context::Context;
use opengl_graphics::GlGraphics;

use crate::coordinate;
use crate::NaturalResourceVariant;

pub trait NaturalResource {
    fn color(&self) -> [f32; 4];

    fn variant(&self) -> NaturalResourceVariant;

    // 埋蔵量
    fn reserves(&self) -> usize;
    fn set_reserves(&mut self, amount: usize);

    // Ok 採取できた資源の量
    fn extract(&mut self, amount: usize) -> Result<usize, &'static str> {
        let reserves = self.reserves();

        return if reserves >= amount {
            self.set_reserves(reserves - amount);
            Ok(amount)
        } else if reserves > 0 {
            let remaining = reserves;
            self.set_reserves(0);
            Ok(remaining)
        } else {
            Err("nothing")
        };
    }

    fn render(&self, gl: &mut GlGraphics, context: &Context, size: &coordinate::Size) {
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
