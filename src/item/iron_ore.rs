mod iron_ore_builder;
pub use iron_ore_builder::*;

mod iron_ore_resource;
pub use iron_ore_resource::*;

mod iron_ore_sign;
pub use iron_ore_sign::*;

mod iron_ore_symbol;
pub use iron_ore_symbol::*;

use crate::item::Material;
use crate::types;

pub struct IronOre {}

impl IronOre {
    const COLOR: types::Color = [0.835294, 0.862745, 0.901961, 1.0];
    const STACK_SIZE: usize = 64;

    pub fn new() -> Self {
        Self {}
    }
}

impl Material for IronOre {}
