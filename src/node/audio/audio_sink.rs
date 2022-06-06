//! Send data to the sound card
//! 
use anyhow::Result;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use crate::runtime::node::SinkNode;


/// Send data to the sound card
/// 
/// Example
/// 
/// ```
/// ```
pub struct AudioSink {
    _stream: cpal::Stream,
}

impl AudioSink {
    pub fn new(sample_rate: u32) -> AudioSink {
        let host = cpal::default_host();

        let device = host
            .default_output_device()
            .ok_or_else(|| anyhow::Error::msg("Default output device is not available")).unwrap();

        let config = cpal::StreamConfig {
            channels: 1,
            sample_rate: cpal::SampleRate(sample_rate),
            buffer_size: cpal::BufferSize::Default,
        };
        // Produce a sinusoid of maximum amplitude.
        let mut sample_clock = 0f32;
        let mut next_value = move || {
            sample_clock = (sample_clock + 1.0) % sample_rate as f32;
            0.25 * (sample_clock * 440.0 * 2.0 * std::f32::consts::PI / sample_rate as f32).sin()
        };
        let err_fn = |err| eprintln!("an error occurred on stream: {}", err);
        let _stream = device.build_output_stream(
            &config,
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                AudioSink::write_data(data, 1, &mut next_value)
            },
            err_fn,
        ).unwrap();
        _stream.play().unwrap();

        AudioSink {_stream}
    }

    fn write_data<T>(output: &mut [T], channels: usize, next_sample: &mut dyn FnMut() -> f32)
    where
        T: cpal::Sample,
    {
        for frame in output.chunks_mut(channels) {
            let value: T = cpal::Sample::from::<f32>(&next_sample());
            for sample in frame.iter_mut() {
                *sample = value;
            }
        }
    }
}

impl SinkNode<f32> for AudioSink {
    fn read_buffer(&mut self, _input_buffer: &[f32]) -> Result<()> {
        Ok(())
    }
}
