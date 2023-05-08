use crate::Item;

pub trait Builder {
    fn build(&self) -> Box<dyn Item>;
}
