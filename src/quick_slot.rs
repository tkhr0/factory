use std::ops::Index;

use crate::item::MaterialVariant;

pub struct QuickSlot {
    items: [Option<MaterialVariant>; 10],
}

impl QuickSlot {
    pub fn items(&self) -> &[Option<MaterialVariant>; 10] {
        &self.items
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn set_item(&mut self, index: usize, item: MaterialVariant) {
        self.items[index] = Some(item);
    }
}

impl Default for QuickSlot {
    fn default() -> Self {
        QuickSlot {
            items: core::array::from_fn(|_| None),
        }
    }
}

impl Index<usize> for QuickSlot {
    type Output = Option<MaterialVariant>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.items[index]
    }
}
