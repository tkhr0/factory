use super::Coal;
use crate::item::Symbol;
use crate::types;

impl Symbol for Coal {
    fn color(&self) -> types::Color {
        Self::COLOR_BODY
    }
}
