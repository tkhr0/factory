use crate::QuickSlot;

#[derive(Default)]
pub struct PlayerState {
    shown_inventory: bool,
    quick_slot: QuickSlot,
}

impl PlayerState {
    pub fn quick_slot(&self) -> &QuickSlot {
        &self.quick_slot
    }

    pub fn quick_slot_mut(&mut self) -> &mut QuickSlot {
        &mut self.quick_slot
    }

    pub fn toggle_inventory(&mut self) {
        self.shown_inventory = !self.shown_inventory;
    }

    pub fn shown_inventory(&self) -> bool {
        self.shown_inventory
    }
}
