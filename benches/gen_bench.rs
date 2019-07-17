// How fast can Sine generator crate samples
// What max frequency can we generate

#[macro_use]
extern crate criterion;

use criterion::Criterion;
use criterion::black_box;

use dsp::generators::{SignalGen, SineGen};


static SAMPLE_FREQ: f32 = 1_000.0;


fn generate_sine(n : usize) -> f32 {
    let gen = SineGen::new(220.0);
    let mut s = 0.0;
    for i in 0..(n*SAMPLE_FREQ as usize) {
        s += gen.sample(i as f32 / SAMPLE_FREQ);
    }
    s
}


fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Sine 1M samples", |b| b.iter(|| generate_sine(black_box(1_000))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);