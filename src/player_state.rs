use crate::QuickSlot;

#[derive(Default)]
pub struct PlayerState {
    quick_slot: QuickSlot,
}

impl PlayerState {
    pub fn quick_slot(&self) -> &QuickSlot {
        &self.quick_slot
    }

    pub fn quick_slot_mut(&mut self) -> &mut QuickSlot {
        &mut self.quick_slot
    }
}
