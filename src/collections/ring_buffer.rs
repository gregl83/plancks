use std::fmt;

use crate::types::event::Event;

const DEFAULT_BUFFER_SIZE: usize = 33;

#[derive(Copy, Clone)]
pub struct RingBuffer {
    buffer: [Event; DEFAULT_BUFFER_SIZE],
}

impl RingBuffer {
    pub fn new() -> RingBuffer {
        RingBuffer {
            buffer: [Event::new(); DEFAULT_BUFFER_SIZE],
        }
    }
}

impl fmt::Debug for RingBuffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = String::new();
        for (i, event) in self.buffer.iter().enumerate() {
            out += &format!("\n\t{}: {:?}", i, event);
        }
        write!(f, "RingBuffer [{}\n]", out)
    }
}