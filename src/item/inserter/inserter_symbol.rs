use super::Inserter;
use crate::item::Symbol;
use crate::types;

impl<const N: usize> Symbol for Inserter<N> {
    fn color(&self) -> types::Color {
        Self::COLOR_BODY
    }
}
