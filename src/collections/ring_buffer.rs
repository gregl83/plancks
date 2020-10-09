use std::fmt;

use crate::types::event::Event;

const BUFFER_SIZE: usize = 1_000_000;
const DEBUG_SLICE_SIZE: usize = 10;

#[derive(Copy, Clone)]
pub struct RingBuffer {
    buffer: [Event; BUFFER_SIZE],
}

impl RingBuffer {
    pub fn new() -> RingBuffer {
        RingBuffer {
            buffer: [Event::new(); BUFFER_SIZE],
        }
    }
}

impl fmt::Debug for RingBuffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let buffer_slice = &self.buffer[0..DEBUG_SLICE_SIZE];
        let mut out = String::new();
        for (i, event) in buffer_slice.iter().enumerate() {
            out += &format!("\n\t{}: {:?}", i, event);
        }
        write!(f, "RingBuffer [{}\n]", out)
    }
}