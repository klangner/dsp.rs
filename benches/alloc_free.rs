// dsp library tries to minimize number of allocations.
// Is it worth it?

#[macro_use]
extern crate criterion;

use criterion::Criterion;
use criterion::black_box;

use dsp::generators::{SignalGen, SineGen};


static SAMPLE_FREQ: f32 = 1000.0;
static SIGNAL_LENGTH: usize = 1;


fn generate_sine(n : usize) -> f32 {
    let mut gen = SineGen::new(4.0, SAMPLE_FREQ);
    let mut s = 0.0;
    for _ in 0..(n*SAMPLE_FREQ as usize) {
        s = gen.next_sample();
    }
    s
}


fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("alloc free 1K", |b| b.iter(|| generate_sine(black_box(SIGNAL_LENGTH))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);