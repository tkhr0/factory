use super::IronOre;
use crate::item::Symbol;
use crate::types;

impl Symbol for IronOre {
    fn color(&self) -> types::Color {
        Self::COLOR
    }
}
