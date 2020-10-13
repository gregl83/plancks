use std::fmt;

use crate::types::event::Event;

const BUFFER_SIZE: usize = 1_000_000;
const DEBUG_SLICE_SIZE: usize = 10;

// operate in series or parallel?
// maintain ordering of consumption
// api for accessing the next element of stage (index)

// event 1: stage: a
// event 2: stage c
// event 3: stage b

// states are array with index? maybe states doesn't need to be dynamic?

// index per stage (locking at some point for ranges)

// stage implies ordering
// state implies how it exists
// ring_buffer.next_in_state(state)
// ring_buffer.idx = state{  }


// States in Ring Buffer
// 1. journal
// 2. replicate
// 3. unmarshall


// state index?
// state a
// state b
// state c
//

#[derive(Copy, Clone)]
pub struct BufferIndex {
    pub key: usize,
}

impl BufferIndex {
    pub fn new() -> BufferIndex {
        BufferIndex {
            key: 0
        }
    }
}

// todo
// write index
// state index
// read index

#[derive(Copy, Clone)]
pub struct RingBuffer {
    // buffer of events
    buf: [
        Event;
        BUFFER_SIZE
    ],

    // index for reads
    idx_r: BufferIndex,

    // index for writes
    idx_w: BufferIndex,
}

impl RingBuffer {
    pub fn new() -> RingBuffer {
        RingBuffer {
            buf: [
                Event::new();
                BUFFER_SIZE
            ],
            idx_r: BufferIndex::new(),
            idx_w: BufferIndex::new(),
        }
    }

    pub fn len(&self) -> usize { self.buf.len() }

    pub fn write(&self, _event: Event) {
        // fixme - implement lifetime parameter
        // self.buf[self.idx_w.key].write(); // fixme - chose write type
        // self.idx_w.key += 1;
    }
}

impl Iterator for RingBuffer {
    type Item = Event;

    fn next(&mut self) -> Option<Self::Item> {
        // todo - return ready values
        if self.idx_r.key == self.buf.len() {
            self.idx_r.key = 0;
        }
        let res = Some(self.buf[self.idx_r.key]);
        self.idx_r.key += 1;
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