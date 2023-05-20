use super::Inserter;
use crate::item::Resource;

impl<const N: usize> Resource for Inserter<N> {
    fn stack_size(&self) -> usize {
        Self::STACK_SIZE
    }
}
