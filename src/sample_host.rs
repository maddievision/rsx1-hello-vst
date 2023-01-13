const EVENT_BUFFER_SIZE: usize = 512;

use crossbeam::channel::Sender;
use std::{
    path::Path,
    sync::{Arc, Mutex},
};
use vst::{
    event::MidiEvent,
    host::{Host, HostBuffer, PluginInstance, PluginLoader},
    prelude::{Plugin, SendEventBuffer},
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
    frame_size: usize,
}

impl VstHost {
    pub fn new(sample_rate: f32, frame_size: usize) -> Self {
        let host = Arc::new(Mutex::new(SampleHost));
        VstHost {
            host,
            devices: vec![],
            sample_rate,
            frame_size,
        }
    }

    pub fn create_device(
        &mut self,
        device_name: String,
        plugin_name: String,
        preset_path: Option<String>,
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

        if let Some(preset_path) = preset_path {
            println!("Loading device {} preset: {}", device_idx, preset_path);
            let preset_bytes = std::fs::read(preset_path).expect("cannot load preset file");
            plugin
                .get_parameter_object()
                .load_preset_data(&preset_bytes);
        }

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

    pub fn process_audio(&mut self, tx: Sender<Vec<f32>>, device_events: &Vec<Vec<MidiEvent>>) {
        let mut frame: Vec<f32> = vec![0.0; self.frame_size * 2];
        let mut send_event_buffer = SendEventBuffer::new(EVENT_BUFFER_SIZE);

        for (idx, device) in self.devices.iter_mut().enumerate() {
            let events = &device_events[idx];
            if events.len() > 0 {
                send_event_buffer.store_events(events.as_slice());
                device.plugin.process_events(send_event_buffer.events());
            }

            /* TODO: it's not good we're creating this every frame lol */
            let mut host_buffer: HostBuffer<f32> = HostBuffer::new(
                device.plugin.get_info().inputs as usize,
                device.plugin.get_info().outputs as usize,
            );

            let inputs = vec![vec![0.0; self.frame_size]; host_buffer.input_count()];
            let mut outputs = vec![vec![0.0; self.frame_size]; host_buffer.output_count()];
            let mut audio_buffer = host_buffer.bind(&inputs, &mut outputs);

            device.plugin.process(&mut audio_buffer);
            let (_, outputs) = audio_buffer.split();

            for c in 0..2 {
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
