use std::fmt;

pub trait Resource {
    fn stack_size(&self) -> usize;
}

impl fmt::Debug for Box<dyn Resource> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Resource").finish()
    }
}
