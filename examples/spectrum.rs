use gnuplot::{Figure, Color, Caption};
use num_complex::Complex32;
use dsp::ComplexBuffer;
use dsp::generators::{SignalGen, ChirpGen};
use dsp::fft::{ForwardFFT};
use dsp::spectrums::max_freq;


const SAMPLE_RATE: usize = 44100;
const SAMPLE_SIZE: usize = 2048;


fn main() {
    let mut gen = ChirpGen::new(10.0, 200.0, 1.0, SAMPLE_RATE);
    let mut ft = ForwardFFT::new(SAMPLE_SIZE);
    let mut spectrum: ComplexBuffer = vec![Complex32::new(0.0, 0.0); SAMPLE_SIZE];

    let ps: Vec<f32> = (0..20).map(|_| {
        let mut xs = (0..SAMPLE_SIZE).map(|_| Complex32::new(gen.next(), 0.0)).collect();
        ft.process(&mut xs, &mut spectrum);
        max_freq(&spectrum, SAMPLE_RATE)
    }).collect();

    let idx: Vec<f32> = (0..ps.len()).map(|x| (x as f32)/100.0).collect();
    let mut fg = Figure::new();
    fg.axes2d().lines(&idx, &ps, &[Color("blue"), Caption("Frequency in Hz.")]);
    fg.show();
}
