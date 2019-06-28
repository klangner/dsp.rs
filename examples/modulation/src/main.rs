
use gnuplot::{Figure, Color, Caption};
use dsp::signals::{Signal};
use dsp::generators::{cosine};
use dsp::fft::{ForwardFFT};

const N: usize = 4096;


fn main() {
    // Our testing signal has 1Hz
    let signal = cosine(2./(N as f32), 0.).generate((0..N).map(|x| x as f32).collect());
    // Our carrier signal has 20Hz
    let carrier = cosine(20./(N as f32), 0.).generate((0..N).map(|x| x as f32).collect());
    // Modulated signal
    let modulated = signal.modulate(&carrier);

    time_plot(&modulated);
    freq_plot(&signal, &carrier, &modulated);

//    demodulated_plot(&signal, &carrier, &modulated);
}

fn time_plot(signal: &Signal) {
    let xs: Vec<f32> = signal.to_vec().iter().map(|x| x.re).collect();
    let idx: Vec<usize> = (0..N).collect();
    let mut fg = Figure::new();
    fg.set_terminal("x11 size 800, 400","");
    fg.axes2d().lines(&idx, &xs, &[Color("green"), Caption("Modulated signal")]);
    fg.show();

}

fn freq_plot(ds: &Signal, cs: &Signal, ms: &Signal) {
    let idx: Vec<usize> = (0..40).collect();
    let mut fg = Figure::new();
    fg.axes2d()
        .lines(&idx, powers(ds), &[Color("green"), Caption("Source signal")])
        .lines(&idx, powers(cs), &[Color("blue"), Caption("Carrier signal")])
        .lines(&idx, powers(ms), &[Color("red"), Caption("Modulated signal")]);
    fg.show();
}


fn powers(v: &Signal) -> Vec<f32> {
    let mut ft = ForwardFFT::new(N);
    let spectrum = ft.process(&v);
    spectrum.to_vec().iter().map(|x| x.re).collect()
}

//fn demodulated_plot(source: &Vector, carrier: &Vector, modulated: &Vector) {
//    let mut ft = FourierTransform::forward(N, N);
//    let mut fti = FourierTransform::inverse(N, N);
//    let demodulated = modulated.multiply(carrier);
//    // Convert to freq domain to apply filter
//    let ps = ft.process(&demodulated);
//    let ds = fti.process(&ps);
//    let xs: Vec<f32> = source.to_vec().iter().map(|x| x.re).collect();
//    let xs2: Vec<f32> = ds.to_vec().iter().map(|x| x.re/(N as f32)).collect();
//    let idx: Vec<usize> = (0..N).collect();
//    let mut fg = Figure::new();
//    fg.set_terminal("x11 size 800, 400","");
//    fg.axes2d()
//        .lines(&idx, xs, &[Color("green"), Caption("Source signal")])
//        .lines(&idx, xs2, &[Color("red"), Caption("Demodulated signal")]);
//    fg.show();
//
//}

