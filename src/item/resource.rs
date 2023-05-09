use std::fmt;

use crate::types;

pub trait Resource {
    fn color(&self) -> types::Color;
    fn stack_size(&self) -> usize;
}

impl fmt::Debug for Box<dyn Resource> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Resource").finish()
    }
}
