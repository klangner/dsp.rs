// dsp library tries to minimize number of allocations.
// Is it worth it?

#[macro_use]
extern crate criterion;

use criterion::Criterion;
use criterion::black_box;

use dsp::SourceNode;
use dsp::generators::SineGen;


static SAMPLE_RATE: usize = 1000;
static BUFFER_COUNT: usize = 1_000;


fn generate_alloc_free(n : usize) -> f32 {
    let mut gen = SineGen::new(4.0, SAMPLE_RATE);
    let mut buffer = vec![0.0; SAMPLE_RATE];
    for _ in 0..n {
        gen.next(&mut buffer);
    }
    buffer[0]
}

fn generate_with_alloc(n : usize) -> f32 {
    let mut gen = SineGen::new(4.0, SAMPLE_RATE);
    let buffer = vec![0.0; SAMPLE_RATE];
    let mut bs: Vec<Vec<f32>> = vec![vec![0.0]; n];
    for i in 0..n {
        let mut output = buffer.clone();
        gen.next(&mut output);
        bs[i] = output;
    }
    let mut c = 0.0;
    for i in 0..n {
        c += bs[i][i];
    }
    c
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("alloc free 1K", |b| b.iter(|| generate_alloc_free(black_box(BUFFER_COUNT))));
    c.bench_function("with alloc 1K", |b| b.iter(|| generate_with_alloc(black_box(BUFFER_COUNT))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);