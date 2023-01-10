const FRAME_SIZE: usize = 1024;
const SAMPLE_RATE: u32 = 48000;

use crossbeam::channel::bounded;
use midi_logger::log_midi_event;
use midi_player::MidiPlayer;
use midly::Smf;
use sample_host::VstHost;
use serde::{Deserialize, Serialize};
use std::{env, fs, path::Path, rc::Rc, thread};
use vst::prelude::Plugin;
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

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectDevice {
    pub name: String,
    pub vst_name: String,
    pub preset: Option<String>,
    pub mix_level: f32,
    pub device_filter: usize,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub description: String,
    pub project_file_version: i32,
    pub devices: Vec<ProjectDevice>,
    pub sequence: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("no filename supplied!!")
    }

    let project_base_path = Path::new(&args[1]);
    let project_data = fs::read(project_base_path.join("project.yml")).unwrap();
    let project: Project = serde_yaml::from_slice(&project_data).unwrap();

    let sequence = project_base_path
        .join(project.sequence)
        .to_str()
        .unwrap()
        .to_string();

    println!("Loaded project: {}", project_base_path.to_str().unwrap());
    println!("Name: {}", project.name);
    println!("{}", project.description);

    let (tx, rx) = bounded::<Vec<f32>>(8);
    let _stream = audio_host::start(rx.clone(), SAMPLE_RATE, FRAME_SIZE as u32);

    let mut vst_host = VstHost::new(SAMPLE_RATE as f32, FRAME_SIZE);

    for project_device in project.devices {
        let preset_path = match project_device.preset {
            Some(preset) => Some(project_base_path.join(preset).to_str().unwrap().to_string()),
            None => None,
        };

        vst_host.create_device(
            project_device.name,
            project_device.vst_name,
            preset_path,
            project_device.mix_level,
            project_device.device_filter,
        );
    }

    let event_loop = EventLoop::new();
    let windows: Vec<Window> = vst_host
        .devices
        .iter()
        .map(|_| WindowBuilder::new().build(&event_loop).unwrap())
        .collect();

    let mut last_position: Option<PhysicalPosition<i32>> = Some(PhysicalPosition { x: 1200, y: 5 });

    for (i, device) in vst_host.devices.iter_mut().enumerate() {
        let window = &windows[i];
        let editor = device.plugin.get_editor();
        if let Some(mut x) = editor {
            x.open(window.ns_view());
            let (w, h) = x.size();
            window.set_resizable(false);
            window.set_inner_size(LogicalSize::new(w as f32, h as f32));
            window.set_title(&format!(
                "{} ({}: {})",
                device.name,
                i,
                &device.plugin.get_info().name,
            ));
            if let Some(PhysicalPosition { x, y }) = last_position {
                // position window below/right from the previous
                window.set_outer_position(PhysicalPosition::new(x, y + 60));
                last_position = Some(PhysicalPosition::new(x, y + 60));
            } else if let Ok(PhysicalPosition { x, y }) = window.outer_position() {
                last_position = Some(PhysicalPosition::new(x, y));
            }
        }
    }

    let _vst_processing_thread = thread::spawn(move || {
        let bytes = fs::read(sequence).unwrap();
        let smf = Smf::parse(&bytes).unwrap();
        let sequence: Rc<midi_player::MidiSequence> = Rc::new(smf.into());

        let mut midi_players: Vec<MidiPlayer> = vst_host
            .devices
            .iter()
            .map(|d| {
                MidiPlayer::new(
                    sequence.clone(),
                    SAMPLE_RATE as f32,
                    FRAME_SIZE,
                    d.device_filter,
                )
            })
            .collect();

        println!("Starting MIDI player!");

        loop {
            let device_events = midi_players
                .iter_mut()
                .map(|mp| mp.get_next_events())
                .collect::<Vec<_>>();

            for (i, events) in device_events.iter().enumerate() {
                for evt in events {
                    log_midi_event(&vst_host.devices[i], evt);
                }
            }

            vst_host.process_audio(tx.clone(), device_events);
        }
    });

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
