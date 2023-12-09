
use dsp::node::generator::Sine;
use dsp::runtime::node::SourceNode;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};


const SAMPLE_RATE: u32 = 44100;
const BUFFER_SIZE: u32 = 1024;


fn setup_audio<D>(mut sound_gen: D) -> Result<cpal::Stream, cpal::BuildStreamError> 
    where D: FnMut(&mut [f32]) + Send + 'static,
{

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
            sound_gen(data);
        },
        move |err| {panic!("cpal stream error {:?}", err);}
    );
     
    stream
    
}


fn main() {
    let mut generator = Sine::new(440., SAMPLE_RATE as usize);

    let stream = setup_audio(move |data: &mut [f32]| {
            generator.write_buffer(data).unwrap();
        },).unwrap();
    stream.play().unwrap();
    
    std::thread::sleep(std::time::Duration::from_millis(1_000));
}