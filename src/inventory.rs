use crate::item::MaterialVariant;

const COLUMNS: usize = 4;
const ROWS: usize = 4;
const SLOTS: usize = COLUMNS * ROWS;

pub struct Inventory {
    items: [InventorySlot; SLOTS],
}

impl Inventory {
    pub const COLUMNS: usize = COLUMNS;
    pub const ROWS: usize = ROWS;

    pub fn items(&self) -> &[InventorySlot; SLOTS] {
        &self.items
    }
}

impl Default for Inventory {
    fn default() -> Self {
        Self {
            items: std::array::from_fn(|i| match i {
                // sample
                0 => InventorySlot::new(MaterialVariant::Coal),
                1 => InventorySlot::new(MaterialVariant::Conveyer),
                _ => Default::default(),
            }),
        }
    }
}

#[derive(Default)]
pub struct InventorySlot {
    variant: Option<MaterialVariant>,
    // count: u32,
}

impl InventorySlot {
    pub fn new(variant: MaterialVariant) -> Self {
        Self {
            variant: Some(variant),
        }
    }

    pub fn variant(&self) -> Option<MaterialVariant> {
        self.variant
    }
}
