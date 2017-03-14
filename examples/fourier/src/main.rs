extern crate gnuplot;
extern crate num_complex;
extern crate dsp;

use gnuplot::{Figure, Color};
use dsp::vectors::{Vector};
use dsp::signal::{cosine, sample};
use dsp::freq::{FourierTransform};

// Dimension
static N: usize = 256;


fn main() {
    // Our testing signal has 4Hz
    let signal = cosine(17.0/(N as f64), 0.);
    let xs = Vector::new(sample(&signal, (0..N).map(|x| x as f64).collect()));
    let mut ft = FourierTransform::forward(N, N);
    let spectrum = ft.process(&xs);
    let mut fti = FourierTransform::inverse(N, N);
    let xs2 = fti.process(&spectrum);
    let idx: Vec<usize> = (0..xs.len()).collect();

    let mut fg = Figure::new();
    let powers: Vec<f64> = spectrum.to_vec().iter().map(|x| x.norm()).collect();
    fg.axes2d().lines(&idx, &powers, &[Color("blue")]);
    fg.show();

    fg = Figure::new();
    let xs_real: Vec<f64> = xs.to_vec().iter().map(|x| x.re).collect();
    let xs2_real: Vec<f64> = xs2.to_vec().iter().map(|x| x.re/(N as f64)).collect();
    fg.axes2d()
        .lines(&idx, xs_real, &[Color("green")])
        .lines(&idx, xs2_real, &[Color("red")]);
    fg.set_terminal("x11 size 800, 400","");
    fg.show();


}
