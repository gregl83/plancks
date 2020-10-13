use std::fmt;

#[derive(Copy, Clone)]
pub struct EventState {
    journaled: bool,
    replicated: bool,
    unmarshalled: bool,
}

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
    locked: bool,
    state: EventState
}

impl Event {
    pub fn new() -> Event {
        Event {
            locked: false,
            state: EventState::new()
        }
    }

    pub fn write(&mut self) {
        self.state = EventState::new();
    }
}

impl fmt::Debug for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Event {{\n\t\tlocked: {:?}\n\t}}", self.locked)
    }
}