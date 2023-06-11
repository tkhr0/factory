use super::IronOre;
use crate::item::Resource;

impl Resource for IronOre {
    fn stack_size(&self) -> usize {
        Self::STACK_SIZE
    }
}
