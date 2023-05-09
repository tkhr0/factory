use crate::item::{Coal, ResourceObj};

pub struct CoalBuilder {}

impl CoalBuilder {
    pub fn new() -> Self {
        Self {}
    }

    pub fn build_resource(&self) -> ResourceObj {
        Box::new(Coal::new())
    }
}
