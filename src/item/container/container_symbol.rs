use super::Container;
use crate::item::Symbol;
use crate::types;

impl<const N: usize> Symbol for Container<N> {
    fn color(&self) -> types::Color {
        Self::COLOR_BODY
    }
}
