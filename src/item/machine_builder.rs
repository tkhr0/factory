use crate::item::Machine;

pub trait MachineBuilder {
    fn build(&self) -> Box<dyn Machine>;
}
