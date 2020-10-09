use std::fmt;

#[derive(Copy, Clone)]
pub struct Event {
    locked: bool
}

impl Event {
    pub fn new() -> Event {
        Event {
            locked: false,
        }
    }
}

impl fmt::Debug for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Event {{\n\t\tlocked: {:?}\n\t}}", self.locked)
    }
}