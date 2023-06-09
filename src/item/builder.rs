use crate::item::Material;

pub trait Builder {
    fn build(&self) -> Box<dyn Material>;
}
