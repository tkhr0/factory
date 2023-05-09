mod builder;
mod fixture;
mod item_variant;
mod resource;
mod sign;

pub use builder::Builder;
pub use fixture::Fixture;
pub use item_variant::ItemVariant;
use resource::Resource;
pub use sign::Sign;

mod container;
pub use container::*;

mod conveyer;
pub use conveyer::*;

mod inserter;
pub use inserter::*;

mod coal;
pub use coal::*;

mod item_factory;
pub use item_factory::ItemFactory;

pub trait Item: Fixture + Sign {}
pub type ResourceObj = Box<dyn Resource>;
