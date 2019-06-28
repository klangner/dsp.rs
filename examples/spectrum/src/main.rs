use gnuplot::{Figure, Color, Caption};
use dsp::generators::{chirp};
use dsp::fft::{ForwardFFT};


const SAMPLE_RATE: usize = 44100;
const SAMPLE_SIZE: usize = 2048;


fn main() {
    let points = (0..SAMPLE_RATE).map(|x| (x as f32)/(SAMPLE_RATE as f32)).collect();
    let signal = chirp(110.0, 880.0, 1.0).generate(points);
    let mut ft = ForwardFFT::new(SAMPLE_SIZE);

    // Calculate FFT every 1/10th of the second
    let ps: Vec<f32> = (0..100).map(|i| {
        let start_pos = i * (SAMPLE_RATE-SAMPLE_SIZE) / 100;
        let xs = signal.slice(start_pos, start_pos+SAMPLE_SIZE);
        let spectrum = ft.process(&xs);
        spectrum.max_freq()
    }).collect();

    let idx: Vec<f32> = (0..ps.len()).map(|x| (x as f32)/100.0).collect();
    let mut fg = Figure::new();
    fg.axes2d().lines(&idx, &ps, &[Color("blue"), Caption("Frequency in Hz.")]);
    fg.show();
}
