use crate::item::Sign;

#[derive(Default)]
pub struct PlayerState {
    quick_slot: QuickSlot,
}

impl PlayerState {
    pub fn initialize(&mut self) {
        self.quick_slot = QuickSlotBuilder::new().build();
    }

    pub fn quick_slot(&self) -> &QuickSlot {
        &self.quick_slot
    }
}

pub struct QuickSlot {
    items: [Option<Box<dyn Sign>>; 10],
}

impl QuickSlot {
    pub fn items(&self) -> &[Option<Box<dyn Sign>>; 10] {
        &self.items
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }
}

impl Default for QuickSlot {
    fn default() -> Self {
        QuickSlot {
            items: core::array::from_fn(|_| None),
        }
    }
}

pub struct QuickSlotBuilder {
    items: Option<[Option<Box<dyn Sign>>; 10]>,
}

impl QuickSlotBuilder {
    pub fn new() -> Self {
        Self {
            items: Some(core::array::from_fn(|_| None)),
        }
    }

    pub fn build(&mut self) -> QuickSlot {
        QuickSlot {
            items: self.items.take().unwrap(),
        }
    }
}
