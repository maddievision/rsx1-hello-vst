const PLUGIN_PATH: &str =
    "/Library/Audio/Plug-Ins/VST/chipsynth SFC.vst/Contents/MacOS/chipsynth SFC";
const PRESET_PATH: &str = "data/schala.fxb";
const OUTPUT_DIR: &str = "out";
const OUTPUT_PATH: &str = "out/schala.wav";
const FRAME_SIZE: usize = 256;
const CHANNELS: usize = 2;
const SAMPLE_RATE: f32 = 48000.0;
const EVENT_BUFFER_SIZE: usize = 256;
const MAX_BUFFERED_FRAMES: usize = 8;

use vst::host::{Host, PluginInstance};
use cpal::{StreamConfig, Device, Stream};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use vst::prelude::AudioBuffer;
use std::path::Path;
use std::sync::{Arc, Mutex};

use vst::buffer::SendEventBuffer;
use vst::event::MidiEvent;
use vst::host::{HostBuffer, PluginLoader};
use vst::plugin::Plugin;

use ringbuf::HeapRb;

use crate::timed_event::TimedEventPlayer;
use crate::schala::build_schala;

pub struct SampleHost;

impl Host for SampleHost {
    fn automate(&self, _index: i32, _value: f32) {
        // no op
    }
}

pub struct AudioHost<'a> {
    is_first_frame: &'a bool,
    event_player: &'a TimedEventPlayer<'a>,
    events: &'a Vec<MidiEvent>,
    send_event_buffer: &'a SendEventBuffer,
    plugin: &'a PluginInstance,
    audio_buffer: &'a AudioBuffer<'a, f32>,
    outputs: &'a Vec<Vec<f32>>,
    device: &'a Device,
    stream: &'a Stream,
    config: &'a StreamConfig,
    rb: HeapRb::<Vec<f32>>,
}


impl<'a> AudioHost<'a> {
    pub fn new() -> Self {
        let host = cpal::default_host();
        let device = host.default_output_device().expect("no output device available");
        let config = StreamConfig {
            channels: 2,
            sample_rate: cpal::SampleRate(SAMPLE_RATE as u32),
            buffer_size: cpal::BufferSize::Fixed(FRAME_SIZE as u32),
        };
        // let err_fn = |err| eprintln!("an error occurred on the output audio stream: {}", err);
        // // let sample_format = supported_config.sample_format();
        // let config = supported_config.into();
    
        let vst_host = Arc::new(Mutex::new(SampleHost));
        let path = Path::new(PLUGIN_PATH);
    
        let mut loader = PluginLoader::load(path, vst_host.clone()).unwrap();
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
    
        let mut vst_host_buffer: HostBuffer<f32> = HostBuffer::new(0, CHANNELS);
        let inputs = vec![vec![0.0; FRAME_SIZE]; 0];
        let mut outputs = vec![vec![0.0; FRAME_SIZE]; CHANNELS];
        let mut audio_buffer = vst_host_buffer.bind(&inputs, &mut outputs);
    
        let mut send_event_buffer = SendEventBuffer::new(EVENT_BUFFER_SIZE);
    
        const STEP: usize = 16;
    
        let sequence = build_schala(STEP);
        
        let mut events: Vec<MidiEvent> = Vec::with_capacity(EVENT_BUFFER_SIZE);
        let mut event_player = TimedEventPlayer::new(&sequence);
        let mut is_first_frame = false;

        AudioHost {
            is_first_frame: &is_first_frame,
            event_player: &event_player,
            events: &events,
            send_event_buffer: &send_event_buffer,
            plugin: &plugin,
            audio_buffer: &audio_buffer,
            outputs: &outputs,
            config: &config,
            device: &device,
            stream: &stream,
        }
    }

    pub fn start(&mut self) {
        println!("Starting MIDI event sequence and output capture");
        self.plugin.resume();

        self.stream = &self.device.build_output_stream(
            self.config,
            move |data, info| self.audio_callback(data, info),
            move |err| eprintln!("an error occurred on the output audio stream: {}", err),
        ).unwrap();
        
        self.stream.play().unwrap();
    
        println!("Playing audio!");
    }

    fn process_audio(&mut self) {
        let (mut prod, _) = self.rb.split();
        let mut frame: Vec<f32> = Vec::with_capacity(FRAME_SIZE * 2);


        /* https://docs.rs/ringbuf/latest/ringbuf/
         * process frames and push onto ring buffer, increase frames
         * do this until we reach max prepared frames (occupied_len > ?)
         */

        match *self.is_first_frame {
            true => self.event_player.play(0, &mut self.events),
            false => self.event_player.play(1, &mut self.events),
        }
        *self.is_first_frame = false;

        if self.events.len() > 0 {
            self.send_event_buffer.store_events(self.events.as_slice());
            self.plugin.process_events(self.send_event_buffer.events());
        }

        self.plugin.process(&mut self.audio_buffer);
        for (c, channel) in self.outputs.iter().enumerate() {
            for (i, sample) in channel.iter().enumerate() {
                data[i * 2 + c] = *sample;
            }
        }
    }

    fn audio_callback(&mut self, data: &mut [f32], _: &cpal::OutputCallbackInfo) {
        let (_, mut cons) = self.rb.split();
        match cons.pop() {
            Some(frame) => {
                data.copy_from_slice(frame.as_slice())
            },
            None => (), /* buffer underrun ???? ( cover your ears 🙉 ) */
        }
    }
}
