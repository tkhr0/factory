use crate::item::MaterialVariant;
use crate::Inventory;
use crate::QuickSlot;

#[derive(Default)]
pub struct PlayerState {
    inventory: Inventory,
    shown_inventory: bool,

    quick_slot: QuickSlot,

    holding_item: Option<MaterialVariant>,
}

impl PlayerState {
    pub fn quick_slot(&self) -> &QuickSlot {
        &self.quick_slot
    }

    pub fn quick_slot_mut(&mut self) -> &mut QuickSlot {
        &mut self.quick_slot
    }

    pub fn inventory(&self) -> &Inventory {
        &self.inventory
    }

    pub fn toggle_inventory(&mut self) {
        self.shown_inventory = !self.shown_inventory;
    }

    pub fn shown_inventory(&self) -> bool {
        self.shown_inventory
    }

    pub fn hold_item(&mut self, item: MaterialVariant) {
        self.holding_item = Some(item);
    }

    pub fn holding_item(&self) -> &Option<MaterialVariant> {
        &self.holding_item
    }

    pub fn release_item(&mut self) {
        self.holding_item = None;
    }
}
