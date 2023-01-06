const PLUGIN_PATH: &str =
    "/Library/Audio/Plug-Ins/VST/chipsynth SFC.vst/Contents/MacOS/chipsynth SFC";
const PRESET_PATH: &str = "data/schala.fxb";
const OUTPUT_DIR: &str = "out";
const OUTPUT_PATH: &str = "out/schala.wav";
const FRAME_SIZE: usize = 256;
const CHANNELS: usize = 2;
const SAMPLE_RATE: f32 = 48000.0;
const EVENT_BUFFER_SIZE: usize = 256;

extern crate vst;

use std::fs;
use std::fs::File;
use std::path::Path;
use std::sync::{Arc, Mutex};
use wav::BitDepth;
use wav::Header;

use vst::buffer::SendEventBuffer;
use vst::event::MidiEvent;
use vst::host::{HostBuffer, PluginLoader};
use vst::plugin::Plugin;

use crate::timed_event::TimedEventPlayer;
pub mod timed_event;

use crate::sample_host::SampleHost;
pub mod sample_host;

use crate::schala::build_schala;
pub mod schala;

pub mod cmd_sequence;

fn main() {
    let host = Arc::new(Mutex::new(SampleHost));
    let path = Path::new(PLUGIN_PATH);

    let mut loader = PluginLoader::load(path, host.clone()).unwrap();
    let mut plugin = loader.instance().unwrap();

    println!("Loaded plugin: {}", plugin.get_info().name);
    plugin.set_sample_rate(SAMPLE_RATE);

    println!("Loading preset: {}", PRESET_PATH);
    let preset_bytes = std::fs::read(PRESET_PATH).expect("cannot load preset file");
    plugin
        .get_parameter_object()
        .load_preset_data(&preset_bytes);

    plugin.init();
    println!("Initialized instance!");

    println!("Setting up audio and event buffers");

    let mut host_buffer: HostBuffer<f32> = HostBuffer::new(0, CHANNELS);
    let inputs = vec![vec![0.0; FRAME_SIZE]; 0];
    let mut outputs = vec![vec![0.0; FRAME_SIZE]; CHANNELS];
    let mut audio_buffer = host_buffer.bind(&inputs, &mut outputs);

    let mut send_event_buffer = SendEventBuffer::new(EVENT_BUFFER_SIZE);

    const STEP: usize = 16;

    let sequence = build_schala(STEP);

    /* 6 bars of music */
    const FRAME_COUNT: usize = STEP * 192;

    println!("Starting MIDI event sequence and output capture");
    plugin.resume();

    let mut collected: Vec<f32> = vec![0.0; FRAME_COUNT * FRAME_SIZE * 2];
    let mut events: Vec<MidiEvent> = Vec::with_capacity(EVENT_BUFFER_SIZE);
    let mut event_player = TimedEventPlayer::new(&sequence);

    for f in 0..FRAME_COUNT {
        match f {
            0 => event_player.play(0, &mut events),
            _ => event_player.play(1, &mut events),
        }

        if events.len() > 0 {
            send_event_buffer.store_events(events.as_slice());
            plugin.process_events(send_event_buffer.events());
        }

        plugin.process(&mut audio_buffer);
        for (c, channel) in outputs.iter().enumerate() {
            for (i, sample) in channel.iter().enumerate() {
                collected[f * FRAME_SIZE * 2 + i * 2 + c] = *sample;
            }
        }
    }

    println!("Writing output to file {}: ", OUTPUT_PATH);

    fs::create_dir_all(OUTPUT_DIR).expect("could not create output dir");
    let header = Header::new(
        wav::WAV_FORMAT_IEEE_FLOAT,
        CHANNELS as u16,
        SAMPLE_RATE as u32,
        32,
    );
    let data = BitDepth::ThirtyTwoFloat(collected);

    let mut out_file = File::create(Path::new(OUTPUT_PATH)).expect("cannot create output file");
    wav::write(header, &data, &mut out_file).expect("cannot write to output file");

    println!("Closing instance...");
    drop(plugin);
}
