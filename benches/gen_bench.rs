// How fast can Sine generator crate samples
// What max frequency can we generate

#[macro_use]
extern crate criterion;

use criterion::Criterion;

use dsp::runtime::node::SourceNode;
use dsp::node::generator::*;


fn criterion_benchmark(c: &mut Criterion) {
    let mut signal = Sinusoid::new(1_000.0, 2048);
    let mut buffer = vec![0.0;1024];
    c.bench_function("Sine generator", |b| {
        b.iter(|| signal.write_buffer(&mut buffer))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);