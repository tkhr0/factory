use crate::item::Material;

pub trait MaterialBuilder {
    fn build(&self) -> Box<dyn Material>;
}
