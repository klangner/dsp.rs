extern crate gnuplot;
extern crate num_complex;
extern crate dsp;

use gnuplot::{Figure, Color};
use dsp::signals::{Signal};
use dsp::generators::{cosine};
use dsp::fft::{ForwardFFT};


// Dimension
static N: usize = 100;


fn main() {
    let signal = cosine(4., 0.).generate((0..N).map(|x| (x as f64)/(N as f64)).collect());
//    plot_signal(&signal);
    plot_fft(&signal);
}


//fn plot_signal(signal: &Signal) {
//    let idx: Vec<usize> = (0..signal.len()).collect();
//    let xs_real: Vec<f64> = signal.to_vec().iter().map(|x| x.re).collect();
//
//    let mut fg = Figure::new();
//    fg.axes2d().lines(&idx, xs_real, &[Color("red")]);
//    fg.set_terminal("x11 size 800, 400","");
//    fg.show();
//
//}

fn plot_fft(signal: &Signal) {
    let mut ft = ForwardFFT::new(N);
    let spectrum = ft.process(&signal);
    let idx: Vec<usize> = (0..signal.len()).collect();
    let powers: Vec<f64> = spectrum.to_vec().iter().map(|x| x.norm()).collect();
    println!("X[4] = {:?}", powers[4]);

    let mut fg = Figure::new();
    fg.axes2d().lines(&idx, &powers, &[Color("blue")]);
    fg.set_terminal("x11 size 800, 400","");
    fg.show();

}