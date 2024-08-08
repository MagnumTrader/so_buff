use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use so_buff::Buffer;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("my_buff 10", |b| b.iter(||my_buffer::<10>(black_box(10))));
    c.bench_function("regular vector 10", |b| b.iter(||vector(black_box(10))));

    c.bench_function("my_buff 100", |b| b.iter(||my_buffer::<100>(black_box(100))));
    c.bench_function("regular vector 100", |b| b.iter(||vector(black_box(100))));

    c.bench_function("my_buff 1000", |b| b.iter(||my_buffer::<1000>(black_box(1000))));
    c.bench_function("regular vector 1000", |b| b.iter(||vector(black_box(1000))));

    c.bench_function("my_buff 10000", |b| b.iter(||my_buffer::<10000>(black_box(10000))));
    c.bench_function("regular vector 10000", |b| b.iter(||vector(black_box(10000))));

}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

fn my_buffer<const N: usize>(items: usize) {
    let mut buffer: Buffer<i32, N> = Buffer::new();
    for i in 0..items {
        let _ = buffer.push(i as i32);
    }
    let mut sum = 0;
    for message in buffer {
        sum += message;
    }
    black_box(sum);
}

fn vector(items: usize) {

    let mut buffer = Vec::with_capacity(items);
    for i in 0..items {
        buffer.push(i);
    }
    let mut sum = 0;
    for message in buffer {
        sum += message;
    }
    black_box(sum);
}
