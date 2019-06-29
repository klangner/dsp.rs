use gnuplot::{Figure, Color};
use num_complex::Complex32;
use dsp::generators::{SignalGen, SineGen};
use dsp::fft::{ForwardFFT, InverseFFT};


fn main() {
    let sample_rate: usize = 256;
    let mut gen = SineGen::new(4.0, sample_rate);
    let mut buffer = vec![0.0; sample_rate];
    gen.next(&mut buffer);

    let mut input = buffer.iter().map(|&x| Complex32::new(x, 0.0)).collect();
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

    fg = Figure::new();
    let out2_real: Vec<f32> = output2.iter().map(|&x| x.re/(sample_rate as f32)).collect();
    fg.axes2d()
        .lines(&idx, buffer, &[Color("green")])
        .lines(&idx, out2_real, &[Color("red")]);
    fg.show();
}
