
use dsp::core::generator::Sine;
use dsp::node::SourceNode;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};


const SAMPLE_RATE: u32 = 44100;
const BUFFER_SIZE: u32 = 1024;


fn main() {
    let mut generator = Sine::new(440., SAMPLE_RATE as usize);

    // Init output device
    let host = cpal::default_host();

    let device = host
        .default_output_device()
        .ok_or_else(|| anyhow::Error::msg("Default output device is not available")).unwrap();

    let config = cpal::StreamConfig {
        channels: 1,
        sample_rate: cpal::SampleRate(SAMPLE_RATE),
        buffer_size: cpal::BufferSize::Fixed(BUFFER_SIZE),
    };

    let stream = device.build_output_stream(
        &config,
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            generator.write_buffer(data).unwrap();
        },
        move |err| {panic!("cpal stream error {:?}", err);},
        None
    ).unwrap();
    
    stream.play().unwrap();
    
    std::thread::sleep(std::time::Duration::from_millis(1_000));
}