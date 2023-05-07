use crate::item::ItemVariant;

pub struct QuickSlot {
    items: [Option<ItemVariant>; 10],
    selected: usize,
}

impl QuickSlot {
    pub fn items(&self) -> &[Option<ItemVariant>; 10] {
        &self.items
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn select(&mut self, index: usize) {
        self.selected = index;
    }

    pub fn selected(&self) -> usize {
        self.selected
    }

    pub fn set_item(&mut self, index: usize, item: ItemVariant) {
        self.items[index] = Some(item);
    }

    pub fn selected_item(&self) -> Option<ItemVariant> {
        self.items[self.selected]
    }
}

impl Default for QuickSlot {
    fn default() -> Self {
        QuickSlot {
            items: core::array::from_fn(|_| None),
            selected: 0,
        }
    }
}
