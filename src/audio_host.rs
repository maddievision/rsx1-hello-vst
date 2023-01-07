use crossbeam::channel::Receiver;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Stream, StreamConfig};

pub fn start(rx: Receiver<Vec<f32>>, sample_rate: u32, frame_size: u32) -> Stream {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("no output device available");
    let config = StreamConfig {
        channels: 2,
        sample_rate: cpal::SampleRate(sample_rate),
        buffer_size: cpal::BufferSize::Fixed(frame_size),
    };

    println!("Starting audio stream!");

    let stream = device
        .build_output_stream(
            &config,
            move |data, _| {
                // println!("check for frame");
                match rx.try_recv() {
                    Ok(frame) => {
                        data.copy_from_slice(&frame);
                        // println!("received frame!");
                    }
                    Err(_) => (), /* buffer underrun ???? ( cover your ears 🙉 ) */
                }
            },
            move |err| eprintln!("an error occurred on the output audio stream: {}", err),
        )
        .unwrap();

    stream.play().unwrap();

    println!("Playing audio!");
    stream
}
