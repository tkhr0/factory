use crate::item::Builder;
use crate::ItemBuilders;

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

pub struct QuickSlot<'a> {
    builders: [Option<&'a dyn Builder>; 10],
}

impl<'a> QuickSlot<'a> {
    pub fn new(builders: &'a ItemBuilders) -> Self {
        let mut item_builders: [Option<&dyn Builder>; 10] = core::array::from_fn(|_| None);

        item_builders[0] = Some(&builders.conveyer);
        item_builders[1] = Some(&builders.container);

        Self {
            builders: item_builders,
        }
    }

    pub fn builders(&self) -> &[Option<&'a dyn Builder>; 10] {
        &self.builders
    }

    pub fn len(&self) -> usize {
        self.builders.len()
    }
}

impl<'a> Default for QuickSlot<'a> {
    fn default() -> Self {
        QuickSlot {
            builders: core::array::from_fn(|_| None),
        }
    }
}
