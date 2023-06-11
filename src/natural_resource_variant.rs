use std::fmt;

use crate::item::MaterialVariant;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NaturalResourceVariant {
    Coal,
    IronOre,
}

impl fmt::Display for NaturalResourceVariant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Coal => "Coal",
                Self::IronOre => "Iron Ore",
            }
        )
    }
}

impl From<NaturalResourceVariant> for MaterialVariant {
    fn from(variant: NaturalResourceVariant) -> Self {
        match variant {
            NaturalResourceVariant::Coal => Self::Coal,
            NaturalResourceVariant::IronOre => Self::IronOre,
        }
    }
}
