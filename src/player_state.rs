use crate::ItemBuilders;
use crate::QuickSlot;

#[derive(Default)]
pub struct PlayerState<'a> {
    quick_slot: QuickSlot<'a>,
}

impl<'a> PlayerState<'a> {
    pub fn initialize(&mut self, builders: &'a ItemBuilders) {
        self.quick_slot = QuickSlot::new(builders);
    }

    pub fn quick_slot(&self) -> &QuickSlot {
        &self.quick_slot
    }
}
