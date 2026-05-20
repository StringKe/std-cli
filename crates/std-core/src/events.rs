use crate::CoreError;
use std_types::StdEvent;

#[derive(Clone, Default)]
pub struct EventLog {
    events: Vec<StdEvent>,
}

impl EventLog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, event: StdEvent) {
        self.events.push(event);
    }

    pub fn list(&self) -> &[StdEvent] {
        &self.events
    }
}

pub trait EventBus {
    fn publish(&self, event: StdEvent) -> Result<(), CoreError>;
    fn events(&self) -> Result<Vec<StdEvent>, CoreError>;
}
