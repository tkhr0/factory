use crate::item::MaterialVariant;

pub struct QuickSlot {
    items: [Option<MaterialVariant>; 10],
    selected: usize,
}

impl QuickSlot {
    pub fn items(&self) -> &[Option<MaterialVariant>; 10] {
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

    pub fn set_item(&mut self, index: usize, item: MaterialVariant) {
        self.items[index] = Some(item);
    }

    pub fn selected_item(&self) -> Option<MaterialVariant> {
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
