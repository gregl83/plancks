mod collections;
mod types;

use collections::ring_buffer::RingBuffer;

fn main() {
    println!(
        "{:?}",
        RingBuffer::new()
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
