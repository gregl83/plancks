use criterion::{black_box, criterion_group, criterion_main, Criterion};
use plancks::collections::ring_buffer::RingBuffer;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("initialize buffer", |b| b.iter(|| {
        RingBuffer::new();
        black_box(20) // fixme - use black_box with input params to bench
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);