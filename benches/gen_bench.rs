// How fast can Sine generator crate samples
// What max frequency can we generate

#[macro_use]
extern crate criterion;

use criterion::Criterion;
use criterion::black_box;

use dsp::generators::*;


fn generate_sine(n : usize) -> Vec<f32> {
    let signal = sine(n, 50.0, n);
    signal
}


fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Sine generator", |b| {
        b.iter(|| generate_sine(black_box(1_000)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);