extern crate gnuplot;
extern crate num_complex;
extern crate audrey;
extern crate dsp;


use gnuplot::{Figure, Color};


fn main() {
    let mut reader = audrey::open("../sounds/sine_440hz.wav").unwrap();
    let mut samples: Vec<f32> = reader.samples().map(Result::unwrap).collect();

    println!("Samples: {:?}", samples.len());

    samples.truncate(1024);
    let idx: Vec<usize> = (0..samples.len()).collect();
//    let ys: Vec<f64> = xs.iter().map(|x| x.re).collect();
    let mut fg = Figure::new();
    fg.axes2d().lines(&idx, &samples, &[Color("red")]);
    fg.show();
}
