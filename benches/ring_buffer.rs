use criterion::{criterion_group, criterion_main, Criterion};
use plancks::collections::ring_buffer::RingBuffer;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("initialize ring buffer", |b| b.iter(|| {
        RingBuffer::new();
    }));

    c.bench_function("iterate buffer", |b| b.iter(|| {
        let buffer = RingBuffer::new();
        // fixme - buffer.iter() and iter.next needed
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);