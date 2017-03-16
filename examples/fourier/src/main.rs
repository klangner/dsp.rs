extern crate gnuplot;
extern crate num_complex;
extern crate dsp;

use gnuplot::{Figure, Color};
use dsp::generators::{cosine};
use dsp::fft::{ForwardFFT, InverseFFT};

// Dimension
static N: usize = 256;


fn main() {
    // Our testing signal has 4Hz
    let signal = cosine(17.0/(N as f64), 0.).generate((0..N).map(|x| x as f64).collect());
    let mut ft = ForwardFFT::new(N);
    let spectrum = ft.process(&signal);
    let mut fti = InverseFFT::new(N);
    let xs2 = fti.process(&spectrum);
    let idx: Vec<usize> = (0..signal.len()).collect();

    let mut fg = Figure::new();
    let powers: Vec<f64> = spectrum.to_vec().iter().map(|x| x.norm()).collect();
    fg.axes2d().lines(&idx, &powers, &[Color("blue")]);
    fg.show();

    fg = Figure::new();
    let xs_real: Vec<f64> = signal.to_vec().iter().map(|x| x.re).collect();
    let xs2_real: Vec<f64> = xs2.to_vec().iter().map(|x| x.re/(N as f64)).collect();
    fg.axes2d()
        .lines(&idx, xs_real, &[Color("green")])
        .lines(&idx, xs2_real, &[Color("red")]);
    fg.set_terminal("x11 size 800, 400","");
    fg.show();


}
