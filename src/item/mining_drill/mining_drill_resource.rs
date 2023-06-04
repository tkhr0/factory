use super::MiningDrill;
use crate::item::Resource;

impl<const N: usize> Resource for MiningDrill<N> {
    fn stack_size(&self) -> usize {
        64
    }
}
