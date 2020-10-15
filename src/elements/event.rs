use std::fmt;
use crate::elements::Element;

#[derive(Copy, Clone)]
pub struct EventState {
    journaled: bool,
    replicated: bool,
    unmarshalled: bool,
}

// todo - decide how dynamic to make this
impl EventState {
    pub fn new() -> EventState {
        EventState {
            journaled: false,
            replicated: false,
            unmarshalled: false,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Event {
    state: EventState
}

impl Event {
    pub fn new() -> Self {
        Event {
            state: EventState::new()
        }
    }
}

impl Element for Event {
    fn write(&mut self) {
        self.state = EventState::new();
    }
}

impl fmt::Debug for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Event {{ }}")
    }
}