use super::MiningDrill;
use crate::coordinate;
use crate::item::Sign;
use crate::types;

impl<const N: usize> Sign for MiningDrill<N> {
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
