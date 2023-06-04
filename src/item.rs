use crate::types;

mod builder;
mod fixture;
mod machine_builder;
mod material_builder;
mod material_variant;
mod resource;
mod sign;
mod symbol;

pub use builder::Builder;
pub use fixture::Fixture;
pub use machine_builder::MachineBuilder;
pub use material_builder::MaterialBuilder;
pub use material_variant::MaterialVariant;
pub use resource::Resource;
pub use sign::Sign;
pub use symbol::Symbol;

mod container;
pub use container::*;

mod conveyer;
pub use conveyer::*;

mod inserter;
pub use inserter::*;

mod mining_drill;
pub use mining_drill::*;

mod coal;
pub use coal::*;

mod machine_factory;
pub use machine_factory::MachineFactory;

mod material_factory;
pub use material_factory::MaterialFactory;

pub trait Material: Sign + Symbol + Resource {
    fn color_symbol(&self) -> types::Color {
        Symbol::color(self)
    }
}

pub trait Machine: Material + Fixture {}

impl std::fmt::Debug for Box<dyn Material> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Material").finish()
    }
}
