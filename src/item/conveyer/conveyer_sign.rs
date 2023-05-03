use super::Conveyer;
use crate::item::Sign;
use crate::types;

impl<const N: usize> Sign for Conveyer<N> {
    fn color(&self) -> types::Color {
        Self::COLOR_BODY
    }

    fn size(&self) -> types::Size {
        types::Size {
            width: 50.0,
            height: 50.0,
        }
    }
}
