mod coal_builder;
pub use coal_builder::*;

mod coal_resource;
pub use coal_resource::*;

mod coal_sign;
pub use coal_sign::*;

mod coal_symbol;
pub use coal_symbol::*;

use crate::item::Material;
use crate::types;

pub struct Coal {}

impl Coal {
    const COLOR_BODY: types::Color = [0.0, 0.0, 0.0, 1.0];
    const STACK_SIZE: usize = 64;

    pub fn new() -> Self {
        Self {}
    }
}

impl Material for Coal {}
