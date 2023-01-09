const EVENT_BUFFER_SIZE: usize = 512;

use crossbeam::channel::Sender;
use std::{
    path::Path,
    sync::{Arc, Mutex},
};
use vst::{
    event::MidiEvent,
    host::{Host, PluginInstance, PluginLoader},
    prelude::{AudioBuffer, Plugin, SendEventBuffer},
};

pub struct SampleHost;

impl Host for SampleHost {
    fn automate(&self, _index: i32, _value: f32) {
        // no op
    }
}

pub struct VstHost {
    host: Arc<Mutex<SampleHost>>,
    pub devices: Vec<PluginInstance>,
    sample_rate: f32,
}

impl VstHost {
    pub fn new(sample_rate: f32) -> Self {
        let host = Arc::new(Mutex::new(SampleHost));
        VstHost {
            host,
            devices: vec![],
            sample_rate,
        }
    }

    pub fn create_device(&mut self, plugin_path: String, preset_path: String) -> usize {
        let device_idx = self.devices.len();
        let path = Path::new(&plugin_path);

        let mut loader = PluginLoader::load(path, self.host.clone()).unwrap();
        let mut plugin = loader.instance().unwrap();

        println!("Loaded device {}: {}", device_idx, plugin.get_info().name);
        plugin.set_sample_rate(self.sample_rate);

        println!("Loading device {} preset: {}", device_idx, preset_path);
        let preset_bytes = std::fs::read(preset_path).expect("cannot load preset file");
        plugin
            .get_parameter_object()
            .load_preset_data(&preset_bytes);

        plugin.init();
        plugin.resume();
        println!("Initialized device {}!", device_idx);

        self.devices.push(plugin);

        device_idx
    }

    pub fn process_audio(
        &mut self,
        tx: Sender<Vec<f32>>,
        audio_buffer: &mut AudioBuffer<f32>,
        device_events: Vec<Vec<MidiEvent>>,
    ) {
        let mut frame: Vec<f32> = vec![0.0; audio_buffer.samples() * 2];
        let mut send_event_buffer = SendEventBuffer::new(EVENT_BUFFER_SIZE);

        for (idx, device) in self.devices.iter_mut().enumerate() {
            let events = &device_events[idx];
            if events.len() > 0 {
                send_event_buffer.store_events(events.as_slice());
                device.process_events(send_event_buffer.events());
            }

            device.process(audio_buffer);
            let (_, outputs) = audio_buffer.split();

            for c in 0..outputs.len() {
                let output = outputs.get(c);
                for (i, sample) in output.iter().enumerate() {
                    frame[i * 2 + c] += *sample;
                }
            }
        }

        match tx.send(frame) {
            Ok(_) => (),
            Err(e) => {
                println!("error sending frame {}", e);
            }
        }
    }
}
