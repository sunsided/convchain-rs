use convchain::{ConvChain, ConvChainSample};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let pattern_4x4 = [
        true, true, true, true, //
        true, false, false, false, //
        true, false, true, false, //
        true, false, false, false,
    ];

    let sample = ConvChainSample::new(&pattern_4x4, 4, 4);

    c.bench_function("4x4 r=2 t=1.0 out=32 it=10", |b| {
        let mut chain = ConvChain::new(&sample, 32, 2, 1.0);
        b.iter(|| {
            chain.process(black_box(10));
        })
    });

    c.bench_function("4x4 r=3 t=1.0 out=32 it=10", |b| {
        let mut chain = ConvChain::new(&sample, 32, 3, 1.0);
        b.iter(|| {
            chain.process(black_box(10));
        })
    });

    c.bench_function("4x4 r=2 t=1.0 out=64 it=10", |b| {
        let mut chain = ConvChain::new(&sample, 64, 2, 1.0);
        b.iter(|| {
            chain.process(black_box(10));
        })
    });

    c.bench_function("4x4 r=2 t=1.0 out=64 it=100", |b| {
        let mut chain = ConvChain::new(&sample, 64, 2, 1.0);
        b.iter(|| {
            chain.process(black_box(100));
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
