use super::MiningDrill;
use crate::item::Symbol;
use crate::types;

impl<const N: usize> Symbol for MiningDrill<N> {
    fn color(&self) -> types::Color {
        Self::COLOR_BODY
    }
}
