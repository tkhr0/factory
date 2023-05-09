use super::Coal;
use crate::item::Resource;
use crate::types;

impl Resource for Coal {
    fn color(&self) -> types::Color {
        Self::COLOR_BODY
    }

    fn stack_size(&self) -> usize {
        Self::STACK_SIZE
    }
}
