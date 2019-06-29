use gnuplot::{Figure, Color};
use dsp::generators::{SignalGen, SineGen};
use dsp::fft::{ForwardFFT, InverseFFT};

// Dimension
static N: usize = 256;


fn main() {
    // Our testing signal has 4Hz
    let mut gen = SineGen::new(4.0, 100);
    let mut buffer = vec![0.0; N];
    gen.next(&mut buffer);

    let mut ft = ForwardFFT::new(N);
    let spectrum = ft.process(&buffer);
    let mut fti = InverseFFT::new(N);
    let xs2 = fti.process(&spectrum);
    let idx: Vec<usize> = (0..signal.len()).collect();

    let mut fg = Figure::new();
    let powers: Vec<f32> = spectrum.to_vec().iter().map(|x| x.norm()).collect();
    fg.axes2d().lines(&idx, &powers, &[Color("blue")]);
    fg.show();

    fg = Figure::new();
    let xs_real: Vec<f32> = signal.to_vec().iter().map(|x| x.re).collect();
    let xs2_real: Vec<f32> = xs2.to_vec().iter().map(|x| x.re/(N as f32)).collect();
    fg.axes2d()
        .lines(&idx, xs_real, &[Color("green")])
        .lines(&idx, xs2_real, &[Color("red")]);
    fg.set_terminal("x11 size 800, 400","");
    fg.show();


}
