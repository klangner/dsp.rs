use gnuplot::{Figure, Color};
use num_complex::Complex32;
use dsp::generators::{SignalGen, SineGen};
use dsp::fft::{ForwardFFT, InverseFFT};


fn main() {
    let sample_rate = 256;
    let mut gen = SineGen::new(4.0, sample_rate as f32);
    let mut input = (0..sample_rate).map(|_| Complex32::new(gen.next(), 0.0)).collect();

    let mut output = vec![Complex32::new(0.0, 0.0); sample_rate];
    let mut output2 = vec![Complex32::new(0.0, 0.0); sample_rate];
    let mut fft = ForwardFFT::new(sample_rate);
    let mut ifft = InverseFFT::new(sample_rate);
    fft.process(&mut input, &mut output);
    ifft.process(&mut output, &mut output2);

    let idx: Vec<usize> = (0..sample_rate).collect();
    let powers: Vec<f32> = output.iter().map(|&x| x.norm()).collect();
    let mut fg = Figure::new();
    fg.axes2d().lines(&idx, &powers, &[Color("blue")]);
    fg.show();
}
