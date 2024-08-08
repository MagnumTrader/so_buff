use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use so_buff::Buffer;


fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("my_buff 10", my_buffer(black_box(10)));
    c.bench_function("my_buff 100", my_buffer(black_box(10)));
    c.bench_function("my_buff 1000", my_buffer(black_box(10)));
    c.bench_function("my_buff 10000", my_buffer(black_box(10)));

    c.bench_function("my_buff 10", vector(black_box(10)));
    c.bench_function("my_buff 100", vector(black_box(10)));
    c.bench_function("my_buff 1000", vector(black_box(10)));
    c.bench_function("my_buff 10000", vector(black_box(10)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);



fn my_buffer(items: usize) {

    let mut buffer: Buffer<i32, items> = Buffer::new();

    for i in 0..items {
        buffer.push(i);
    }

    // consume

    buffer.into_iter();

    for message in buffer {
        drop(message);
    }
}

fn vector(items: usize) {

    let mut buffer = Vec::with_capacity(items);

    for i in 0..items {
        buffer.push(i);
    }

    // consume

    buffer.into_iter();

    for message in buffer {
        drop(message);
    }
}
