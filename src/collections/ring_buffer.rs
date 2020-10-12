use std::fmt;

use crate::types::event::Event;

const BUFFER_SIZE: usize = 1_000_000;
const BUFFER_INDEX_DEFAULT: usize = 0;
const DEBUG_SLICE_SIZE: usize = 10;

#[derive(Copy, Clone)]
pub struct RingBuffer {
    buf: [Event; BUFFER_SIZE],
    idx: usize,
}

impl RingBuffer {
    pub fn new() -> RingBuffer {
        RingBuffer {
            buf: [Event::new(); BUFFER_SIZE],
            idx: BUFFER_INDEX_DEFAULT,
        }
    }

    pub fn len(&self) -> usize { self.buf.len() }

    pub fn index(&self) -> usize { self.idx }
}

impl Iterator for RingBuffer {
    type Item = Event;

    fn next(&mut self) -> Option<Self::Item> {
        // todo - return ready values
        if self.idx == self.buf.len() {
            self.idx = BUFFER_INDEX_DEFAULT;
        }
        let res = Some(self.buf[self.idx]);
        self.idx += 1;
        res
    }
}

impl fmt::Debug for RingBuffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let buffer_slice = &self.buf[0..DEBUG_SLICE_SIZE];
        let mut out = String::new();
        for (i, event) in buffer_slice.iter().enumerate() {
            out += &format!("\n\t{}: {:?}", i, event);
        }
        write!(f, "RingBuffer [{}\n]", out)
    }
}