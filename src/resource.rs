#[derive(Debug)]
pub struct Resource {
    stack_size: usize,
}

impl Resource {
    pub fn new(stack_size: usize) -> Self {
        Self { stack_size }
    }

    pub fn stack_size(&self) -> usize {
        self.stack_size
    }
}

impl Default for Resource {
    fn default() -> Self {
        Self::new(64)
    }
}
