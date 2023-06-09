use super::Coal;
use crate::item::Resource;

impl Resource for Coal {
    fn stack_size(&self) -> usize {
        Self::STACK_SIZE
    }
}
