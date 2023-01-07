const PLUGIN_PATH: &str =
    "/Library/Audio/Plug-Ins/VST/chipsynth SFC.vst/Contents/MacOS/chipsynth SFC";
const PRESET_PATH: &str = "data/schala.fxb";
const FRAME_SIZE: usize = 256;
const EVENT_BUFFER_SIZE: usize = 256;

use crossbeam::channel::Sender;
use std::path::Path;
use std::sync::{Arc, Mutex};
use vst::host::{Host, PluginInstance};
use vst::prelude::AudioBuffer;

use vst::buffer::SendEventBuffer;
use vst::event::MidiEvent;
use vst::host::PluginLoader;
use vst::plugin::Plugin;

pub struct SampleHost;

impl Host for SampleHost {
    fn automate(&self, _index: i32, _value: f32) {
        // no op
    }
}

pub struct VstHost {
    host: Arc<Mutex<SampleHost>>,
    plugin: PluginInstance,
}

impl VstHost {
    pub fn new(sample_rate: f32) -> Self {
        let host = Arc::new(Mutex::new(SampleHost));
        let path = Path::new(PLUGIN_PATH);

        let mut loader = PluginLoader::load(path, host.clone()).unwrap();
        let mut plugin = loader.instance().unwrap();

        println!("Loaded plugin: {}", plugin.get_info().name);
        plugin.set_sample_rate(sample_rate);

        println!("Loading preset: {}", PRESET_PATH);
        let preset_bytes = std::fs::read(PRESET_PATH).expect("cannot load preset file");
        plugin
            .get_parameter_object()
            .load_preset_data(&preset_bytes);

        plugin.init();
        plugin.resume();
        println!("Initialized instance!");

        VstHost { host, plugin }
    }

    pub fn process_audio(
        &mut self,
        tx: Sender<Vec<f32>>,
        audio_buffer: &mut AudioBuffer<f32>,
        events: Vec<MidiEvent>,
    ) {
        let mut frame: Vec<f32> = vec![0.0; FRAME_SIZE * 2];
        let mut send_event_buffer = SendEventBuffer::new(EVENT_BUFFER_SIZE);

        if events.len() > 0 {
            send_event_buffer.store_events(events.as_slice());
            self.plugin.process_events(send_event_buffer.events());
        }

        self.plugin.process(audio_buffer);
        let (_, outputs) = audio_buffer.split();

        for c in 0..outputs.len() {
            let output = outputs.get(c);
            for (i, sample) in output.iter().enumerate() {
                frame[i * 2 + c] = *sample;
            }
        }

        // println!("sending frame!");
        match tx.send(frame) {
            Ok(_) => {
                // println!("sent frame!");
            }
            Err(e) => {
                println!("error sending frame {}", e);
            }
        }
    }
}
