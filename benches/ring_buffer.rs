use criterion::{criterion_group, criterion_main, Criterion};
use plancks::collections::ring_buffer::RingBuffer;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("initialize ring buffer", |b| b.iter(|| {
        let _ring_buffer = RingBuffer::new();
    }));

    // todo: throughput / eps (events per second)
    // https://bheisler.github.io/criterion.rs/book/user_guide/advanced_configuration.html#throughput-measurements
    let buffer = RingBuffer::new();
    let buffer_length = buffer.len();
    let mut event_count = 0;
    c.bench_function("iterate ring buffer", |b| b.iter(|| {
        for _event in buffer {
            event_count += 1;
            if event_count >= buffer_length { break }
        }
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);