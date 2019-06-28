use gnuplot::{Figure, Color};
use dsp::signals::{Signal};
use dsp::generators::{cosine};
use dsp::fft::{ForwardFFT};


// Dimension
static N: usize = 100;


fn main() {
    let signal = cosine(4., 0.).generate((0..N).map(|x| (x as f32)/(N as f32)).collect());
//    plot_signal(&signal);
    plot_fft(&signal);
}


//fn plot_signal(signal: &Signal) {
//    let idx: Vec<usize> = (0..signal.len()).collect();
//    let xs_real: Vec<f32> = signal.to_vec().iter().map(|x| x.re).collect();
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
    let powers: Vec<f32> = spectrum.to_vec().iter().map(|x| x.norm()).collect();
    println!("X[4] = {:?}", powers[4]);

    let mut fg = Figure::new();
    fg.axes2d().lines(&idx, &powers, &[Color("blue")]);
    fg.show();

}