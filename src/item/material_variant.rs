use crate::item::{Machine, MachineFactory, Material, MaterialFactory};

#[derive(Debug, Clone, Copy)]
pub enum MaterialVariant {
    Container,
    Conveyer,
    Inserter,

    // resources
    Coal,
}

impl MaterialVariant {
    pub fn as_material(&self) -> Box<dyn Material> {
        MaterialFactory::build(*self)
    }

    pub fn as_machine(&self) -> Option<Box<dyn Machine>> {
        MachineFactory::build(*self)
    }
}
