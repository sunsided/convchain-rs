use convchain::conv_chain;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let pattern_8x8 = [
        true, true, true, true, true, true, true, true, //
        true, false, false, false, true, false, false, false, //
        true, false, true, false, true, false, true, false, //
        true, false, false, false, true, false, false, false, //
        true, true, true, true, true, true, true, true, //
        true, false, false, false, true, false, false, false, //
        true, false, true, false, true, false, true, false, //
        true, false, false, false, true, false, false, false,
    ];

    c.bench_function("8x8 r=2 t=1.0 out=32 it=10", |b| {
        b.iter(|| {
            conv_chain(
                black_box(&pattern_8x8),
                black_box(8),
                black_box(8),
                black_box(2),
                black_box(1.0),
                black_box(32),
                black_box(10),
            )
        })
    });

    c.bench_function("8x8 r=3 t=1.0 out=32 it=10", |b| {
        b.iter(|| {
            conv_chain(
                black_box(&pattern_8x8),
                black_box(8),
                black_box(8),
                black_box(2),
                black_box(1.0),
                black_box(32),
                black_box(10),
            )
        })
    });

    c.bench_function("8x8 r=2 t=1.0 out=64 it=10", |b| {
        b.iter(|| {
            conv_chain(
                black_box(&pattern_8x8),
                black_box(8),
                black_box(8),
                black_box(2),
                black_box(1.0),
                black_box(64),
                black_box(10),
            )
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
