use super::Container;
use crate::item::Resource;

impl<const N: usize> Resource for Container<N> {
    fn stack_size(&self) -> usize {
        Self::STACK_SIZE
    }
}
