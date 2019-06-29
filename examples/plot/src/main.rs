use gnuplot::{Figure, Color};
use clap::{Arg, App};
use dsp::generators::*;


const N: usize = 512;

fn main() {
    // Parse command line arguments
    let args = App::new("Plot signal")
                    .arg(Arg::with_name("Generator")
                        .short("g")
                        .long("generator")
                        .value_name("FILE")
                        .help("Select generator type")
                        .takes_value(true))
                    .get_matches();
    let gen_name = args.value_of("Generator").unwrap_or("sine");
    // Create generator and generate signal
    let mut buffer = vec![0.0; N];
    match gen_name {
        "traingle"  => TriangleGen::new(4.0, N).next(&mut buffer),
        "square"    => SquareGen::new(4.0, N).next(&mut buffer),
        "noise"     => NoiseGen::new(0.4).next(&mut buffer),
        _           => SineGen::new(4.0, N).next(&mut buffer),
    };

    // Plot signal
    let idx: Vec<usize> = (0..N).collect();
    let ys: Vec<f32> = buffer;

    let mut fg = Figure::new();
    fg.axes2d().lines(&idx, &ys, &[Color("red")]);
    fg.show();
}