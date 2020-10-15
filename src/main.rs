mod collections;
mod elements;

use collections::ring_buffer::RingBuffer;
use elements::event::Event;

fn main() {
    let mut buffer = RingBuffer::new(
        Event::new()
    );

    // todo - test streaming iterator

    println!(
        "{:?}",
        buffer
    );

    // todo - need workers for parallelism

    // todo - benchmarks (parallelism, read, writes, etc)

    // todo - memory allocation details/tests

    // todo - overflow panic (cleanup/dump state?)

    // todo - control-flow operations applied (series/parallel); state MUST reflect status there
    // requirements:
    //  - when to consume
    //  - when consumed
    //  - when to overwrite
}
