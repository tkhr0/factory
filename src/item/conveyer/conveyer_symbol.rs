use super::Conveyer;
use crate::item::Symbol;
use crate::types;

impl<const N: usize> Symbol for Conveyer<N> {
    fn color(&self) -> types::Color {
        Self::COLOR_BODY
    }
}
