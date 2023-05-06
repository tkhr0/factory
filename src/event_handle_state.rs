#[must_use]
pub enum EventHandleState {
    Consumed,
    Remaining,
}

impl EventHandleState {
    pub fn consume(&mut self) {
        *self = Self::Consumed;
    }

    pub fn consumed(&self) -> bool {
        matches!(self, Self::Consumed)
    }
}

impl Default for EventHandleState {
    fn default() -> Self {
        Self::Remaining
    }
}
