use crate::item::MaterialVariant;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NaturalResourceVariant {
    Coal,
    IronOre,
}

impl From<NaturalResourceVariant> for MaterialVariant {
    fn from(variant: NaturalResourceVariant) -> Self {
        match variant {
            NaturalResourceVariant::Coal => Self::Coal,
            NaturalResourceVariant::IronOre => Self::IronOre,
        }
    }
}
