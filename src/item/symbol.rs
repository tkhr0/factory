use crate::types;

pub trait Symbol {
    fn color(&self) -> types::Color;
}
