use super::Conveyer;
use crate::item::Resource;

impl<const N: usize> Resource for Conveyer<N> {
    fn stack_size(&self) -> usize {
        Self::STACK_SIZE
    }
}
