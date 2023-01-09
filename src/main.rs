const FRAME_SIZE: usize = 1024;
const SAMPLE_RATE: u32 = 48000;

use crossbeam::channel::bounded;
use midi_logger::log_midi_event;
use midi_player::MidiPlayer;
use midly::Smf;
use sample_host::VstHost;
use std::{fs, rc::Rc, thread};
use vst::{host::HostBuffer, prelude::Plugin};
use winit::{
    dpi::{LogicalSize, PhysicalPosition},
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::macos::WindowExtMacOS,
    window::{Window, WindowBuilder},
};

extern crate vst;

pub mod audio_host;
pub mod midi_logger;
pub mod midi_player;
pub mod sample_host;

fn main() {
    // Load bytes into a buffer

    let (tx, rx) = bounded::<Vec<f32>>(8);
    let _stream = audio_host::start(rx.clone(), SAMPLE_RATE, FRAME_SIZE as u32);

    let mut vst_host = VstHost::new(SAMPLE_RATE as f32);
    vst_host.create_device(
        "/Library/Audio/Plug-Ins/VST/chipsynth SFC.vst/Contents/MacOS/chipsynth SFC".to_string(),
        "data/schala_inst.fxp".to_string(),
    );

    vst_host.create_device(
        "/Library/Audio/Plug-Ins/VST/chipsynth SFC.vst/Contents/MacOS/chipsynth SFC".to_string(),
        "data/schala_perc.fxp".to_string(),
    );

    let event_loop = EventLoop::new();
    let windows: Vec<Window> = vec![
        WindowBuilder::new().build(&event_loop).unwrap(),
        WindowBuilder::new().build(&event_loop).unwrap(),
    ];

    let mut last_position: Option<PhysicalPosition<i32>> = None;

    for (i, device) in vst_host.devices.iter_mut().enumerate() {
        let window = &windows[i];
        let editor = device.get_editor();
        if let Some(mut x) = editor {
            x.open(window.ns_view());
            let (w, h) = x.size();
            window.set_resizable(false);
            window.set_inner_size(LogicalSize::new(w as f32, h as f32));
            window.set_title(&format!("{} (Device {})", &device.get_info().name, i));
            if let Some(PhysicalPosition { x, y }) = last_position {
                // position window below/right from the previous
                window.set_outer_position(PhysicalPosition::new(x + 40, y + 40));
            }
            if let Ok(PhysicalPosition { x, y }) = window.outer_position() {
                last_position = Some(PhysicalPosition::new(x, y));
            }
        }
    }

    let _vst_processing_thread = thread::spawn(move || {
        let bytes = fs::read("data/schala.mid").unwrap();
        let smf = Smf::parse(&bytes).unwrap();
        let sequence: Rc<midi_player::MidiSequence> = Rc::new(smf.into());

        let mut host_buffer: HostBuffer<f32> = HostBuffer::new(0, 2);
        let inputs = vec![vec![0.0; FRAME_SIZE]; 0];
        let mut outputs = vec![vec![0.0; FRAME_SIZE]; 2];
        let mut audio_buffer = host_buffer.bind(&inputs, &mut outputs);
        const STEP: usize = 16;
        let mut midi_players = vec![
            MidiPlayer::new(sequence.clone(), SAMPLE_RATE as f32, FRAME_SIZE, 0),
            MidiPlayer::new(sequence.clone(), SAMPLE_RATE as f32, FRAME_SIZE, 1),
        ];
        println!("Starting MIDI player!");

        loop {
            let device_events = midi_players
                .iter_mut()
                .map(|mp| mp.get_next_events())
                .collect::<Vec<_>>();

            for (i, events) in device_events.iter().enumerate() {
                for evt in events {
                    log_midi_event(i, evt);
                }
            }

            vst_host.process_audio(tx.clone(), &mut audio_buffer, device_events);
        }
    });

    // _vst_processing_thread.join();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if windows.iter().any(|w| window_id == w.id()) => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}
