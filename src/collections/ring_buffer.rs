use std::fmt;
use std::fmt::Debug;

use crate::collections::{Buffer, StreamingIterator};
use crate::elements::Element;

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

#[derive(Clone)]
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

// #[derive(Clone)]
// struct Message {
//     id: u128
// }
//
// #[derive(Clone)]
// struct Bufferr<B: Element + Clone> {
//     buf: Vec<B>
// }
//
// impl Element for Message {
//     fn get_id(&self) -> u128 {
//         1
//     }
// }
//
// impl<B: Element + Clone> Bufferr<B> {
//     pub fn new(element: B) -> Self {
//         Buffer {
//             buf: vec![element; 1000000],
//         }
//     }
// }
//
// let message = Message {
//      id: 1
// };
// let buffer = Buffer::new(message);

#[derive(Clone)]
pub struct RingBuffer<E: Element> {
    // vector of elements (Element trait)
    buf: Vec<E>,

    // index for reads
    idx_r: BufferIndex,

    // index for writes
    idx_w: BufferIndex,
}


impl<E: Element + Clone> RingBuffer<E> {
    pub fn new(el: E) -> Self {
        RingBuffer {
            buf: vec![el; BUFFER_SIZE],
            idx_r: BufferIndex::new(),
            idx_w: BufferIndex::new(),
        }
    }

    pub fn len(&self) -> usize { self.buf.len() }
}

impl<'a, E: Element> Buffer<'a, E> for RingBuffer<E> {
    fn write(&self, el: E) {
        // fixme - implement lifetime parameter
        // self.buf[self.idx_w.key].write(); // fixme - chose write type
        // self.idx_w.key += 1;
    }
}

// FIXME - lifetime parameter on Iterator trait
impl<'a, E: Element> StreamingIterator<'a> for RingBuffer<E> {
    type Item = E;

    fn next(&'a mut self) -> Option<&mut Self::Item> {
        // todo - return ready values
        if self.idx_r.key == self.buf.len() {
            self.idx_r.key = 0;
        }
        let res = Some(&mut self.buf[self.idx_r.key]);
        self.idx_r.key += 1;
        res
    }
}

impl<E: Element + Debug> fmt::Debug for RingBuffer<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let buffer_slice = &self.buf[0..DEBUG_SLICE_SIZE];
        let mut out = String::new();
        for (i, event) in buffer_slice.iter().enumerate() {
            out += &format!("\n\t{}: {:?}", i, event);
        }
        write!(f, "RingBuffer [{}\n]", out)
    }
}