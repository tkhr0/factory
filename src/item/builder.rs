use crate::item::Item;

pub trait Builder {
    fn build(&self) -> Box<dyn Item>;
}
