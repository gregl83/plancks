use criterion::{criterion_group, criterion_main, Criterion};
use plancks::collections::ring_buffer::RingBuffer;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("initialize ring buffer", |b| b.iter(|| {
        RingBuffer::new();
    }));

    let buffer = RingBuffer::new();
    c.bench_function("iterate ring buffer", |b| b.iter(|| {
        let _event = buffer.iterate(); // fixme - buffer.iter() and iter.next needed
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);