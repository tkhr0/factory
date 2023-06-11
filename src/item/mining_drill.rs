mod mining_drill_builder;
pub use mining_drill_builder::MiningDrillBuilder;

mod mining_drill_fixture;
pub use mining_drill_fixture::*;

mod mining_drill_resource;
pub use mining_drill_resource::*;

mod mining_drill_sign;
pub use mining_drill_sign::*;

mod mining_drill_symbol;
pub use mining_drill_symbol::*;

use crate::coordinate;
use crate::item::{Machine, Material};
use crate::types;
use crate::NaturalResource;
use crate::NaturalResourceVariant;
use crate::Slot;

#[derive(Debug)]
pub struct MiningDrill<const N: usize> {
    #[allow(dead_code)]
    name: &'static str,
    slots: [Slot; N],
    cooling_time: f64,
    direction: types::Direction,
}

impl<const N: usize> MiningDrill<N> {
    const COLOR_BODY: types::Color = [0.7, 0.7, 0.7, 1.0];
    const MINING_AMOUNT: usize = 1;
    const MINABLE_NATURAL_RESOURCES: [NaturalResourceVariant; 2] = [
        NaturalResourceVariant::Coal,
        NaturalResourceVariant::IronOre,
    ];

    fn minable(&self, target: &dyn NaturalResource) -> bool {
        if !self.slots.iter().any(|slot| slot.is_empty()) {
            return false;
        }

        for variant in Self::MINABLE_NATURAL_RESOURCES.iter() {
            if target.variant() == *variant {
                return true;
            }
        }

        false
    }

    fn direction(&self) -> &types::Direction {
        &self.direction
    }

    fn width(&self) -> f64 {
        50.0
    }

    fn height(&self) -> f64 {
        50.0
    }

    fn size(&self) -> coordinate::Size {
        coordinate::Size::new(self.width(), self.height())
    }

    fn angle(&self) -> types::Radian {
        self.direction().angle()
    }
}

impl<const N: usize> Material for MiningDrill<N> {}
impl<const N: usize> Machine for MiningDrill<N> {}
