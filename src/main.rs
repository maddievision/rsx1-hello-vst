const PLUGIN_PATH: &str = "/Library/Audio/Plug-Ins/VST/chipsynth SFC.vst/Contents/MacOS/chipsynth SFC";

extern crate vst;

use std::sync::{Arc, Mutex};
use std::path::Path;

use vst::host::{Host, PluginLoader, HostBuffer};
use vst::buffer::SendEventBuffer;
use vst::plugin::Plugin;
use vst::event::{MidiEvent};

struct SampleHost;

impl Host for SampleHost {
    fn automate(&self, index: i32, value: f32) {
        println!("Parameter {} had its value changed to {}", index, value);
    }
}

fn main() {
    let host = Arc::new(Mutex::new(SampleHost));
    let path = Path::new(PLUGIN_PATH);

    let mut loader = PluginLoader::load(path, host.clone()).unwrap();
    let mut plugin = loader.instance().unwrap();

    println!("Loaded {}", plugin.get_info().name);

    plugin.init();
    println!("Initialized instance!");

    let mut host_buffer: HostBuffer<f32> = HostBuffer::new(0, 2);
    let inputs = vec![vec![0.0; 256]; 0];
    let mut outputs = vec![vec![0.0; 256]; 2];
    let mut audio_buffer = host_buffer.bind(&inputs, &mut outputs);

    plugin.resume();

    println!("Sending MIDI");

    let mut send_event_buffer = SendEventBuffer::new(512);

    let event = MidiEvent {
        data: [0x90, 0x3C, 0x7F],
        delta_frames: 0,
        live: true,
        note_length: None,
        note_offset: None,
        detune: 0,
        note_off_velocity: 0,
    };

    send_event_buffer.store_events([event]);
    plugin.process_events(send_event_buffer.events());

    // for n in 1..101 {
    let n = 0;

    println!("Processing Audio");

    plugin.process(&mut audio_buffer);

    println!("Reading output frame {}: ", n);

    for (i, channel) in outputs.iter().enumerate() {
        println!("Channel {}: ", i);
        for sample in channel {
            /* magnify f32 output by 32,767 (to approximate i16 samples, for console output legibitility) */
            print!("{num:.p$}, ", num = sample * 32767.0, p = 0);
        }
        println!("");
    }

    println!("Closing instance...");
    drop(plugin);
}
