use super::IronOre;
use crate::item::Sign;
use crate::types;

impl Sign for IronOre {
    fn color(&self) -> types::Color {
        Self::COLOR
    }

    fn size(&self) -> types::Size {
        types::Size {
            width: 50.0,
            height: 50.0,
        }
    }
}
