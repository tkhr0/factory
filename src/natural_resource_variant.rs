use crate::item::MaterialVariant;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NaturalResourceVariant {
    IronOre,
}

impl From<NaturalResourceVariant> for MaterialVariant {
    fn from(variant: NaturalResourceVariant) -> Self {
        match variant {
            NaturalResourceVariant::IronOre => MaterialVariant::IronOre,
        }
    }
}
