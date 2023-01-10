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

pub struct Device {
    pub name: String,
    pub plugin: PluginInstance,
    pub device_filter: usize,
    pub mix_level: f32,
}

pub struct SampleHost;

impl Host for SampleHost {
    fn automate(&self, _index: i32, _value: f32) {
        // no op
    }
}

pub struct VstHost {
    host: Arc<Mutex<SampleHost>>,
    pub devices: Vec<Device>,
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

    pub fn create_device(
        &mut self,
        device_name: String,
        plugin_name: String,
        preset_path: String,
        mix_level: f32,
        device_filter: usize,
    ) -> usize {
        let device_idx = self.devices.len();
        let path = format!(
            "/Library/Audio/Plug-Ins/VST/{}.vst/Contents/MacOS/{}",
            plugin_name, plugin_name
        );
        let path = Path::new(&path);

        let mut loader = PluginLoader::load(path, self.host.clone()).unwrap();
        let mut plugin = loader.instance().unwrap();

        println!("Loaded device {}: {}", device_name, plugin.get_info().name);
        plugin.set_sample_rate(self.sample_rate);

        println!("Loading device {} preset: {}", device_idx, preset_path);
        let preset_bytes = std::fs::read(preset_path).expect("cannot load preset file");
        plugin
            .get_parameter_object()
            .load_preset_data(&preset_bytes);

        plugin.init();
        plugin.resume();
        println!("Initialized device {}!", device_idx);

        self.devices.push(Device {
            name: device_name,
            plugin,
            device_filter,
            mix_level,
        });

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
                device.plugin.process_events(send_event_buffer.events());
            }

            device.plugin.process(audio_buffer);
            let (_, outputs) = audio_buffer.split();

            for c in 0..outputs.len() {
                let output = outputs.get(c);
                for (i, sample) in output.iter().enumerate() {
                    frame[i * 2 + c] += *sample * device.mix_level;
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
