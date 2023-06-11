use super::Coal;
use crate::coordinate;
use crate::item::Sign;
use crate::types;

impl Sign for Coal {
    fn color(&self) -> types::Color {
        Self::COLOR_BODY
    }

    fn size(&self) -> coordinate::Size {
        coordinate::Size {
            width: 50.0,
            height: 50.0,
        }
    }
}
