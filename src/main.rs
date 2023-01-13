const FRAME_SIZE: usize = 1024;
const SAMPLE_RATE: u32 = 48000;

use crossbeam::channel::bounded;
use midi_logger::{MidiLogFilter, MidiLogger};
use midi_player::MidiPlayer;
use midly::Smf;
use rustop::opts;
use sample_host::VstHost;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::Path,
    rc::Rc,
    sync::{Arc, Mutex},
    thread,
};
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
    let (args, _rest) = opts! {
        synopsis "Simple VST MIDI player";
        opt note_on_log:bool=true, short:'n', desc:"Don’t log note-on messages.";
        opt loops_log:bool=true, short:'l', desc:"Don’t log loop commands.";
        opt note_off_log:bool, short:'o', desc:"Log note-off messages.";
        opt controller_log:bool, short:'c', desc:"Log controller messages.";
        opt pitch_log:bool, short:'p', desc:"Log pitch messages.";
        opt open_vst_editors:bool, short:'e', desc:"Open VST editors.";
        param project_path:String, desc:"Path to project containing project.yml file";
    }
    .parse_or_exit();

    let event_loop = EventLoop::new();
    let main_window = WindowBuilder::new().build(&event_loop).unwrap();
    main_window.set_title("nutrax");
    let mut windows: Vec<Window> = vec![];

    let project_base_path = Path::new(&args.project_path);
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

    let vst_host = Arc::new(Mutex::new(VstHost::new(SAMPLE_RATE as f32, FRAME_SIZE)));

    if let Ok(mut vst_host) = vst_host.lock() {
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

        let mut last_position: Option<PhysicalPosition<i32>> =
            Some(PhysicalPosition { x: 1200, y: 5 });

        if args.open_vst_editors {
            for (i, device) in vst_host.devices.iter_mut().enumerate() {
                let editor = device.plugin.get_editor();
                if let Some(mut x) = editor {
                    let window = WindowBuilder::new().build(&event_loop).unwrap();
                    window.set_title(&format!(
                        "{} ({}: {})",
                        device.name,
                        i,
                        &device.plugin.get_info().name
                    ));
                    x.open(window.ns_view());
                    if let Some(PhysicalPosition { x, y }) = last_position {
                        // position window below/right from the previous
                        window.set_outer_position(PhysicalPosition::new(x, y + 60));
                        last_position = Some(PhysicalPosition::new(x, y + 60));
                    } else if let Ok(PhysicalPosition { x, y }) = window.outer_position() {
                        last_position = Some(PhysicalPosition::new(x, y));
                    }
                    let (w, h) = x.size();
                    window.set_inner_size(LogicalSize::new(w as f32, h as f32));
                    window.set_resizable(false);
                    windows.push(window);
                }
            }
        }
    }

    let vstd = vst_host.clone();
    let vste = vst_host.clone();

    let _vst_processing_thread = thread::spawn(move || {
        let bytes = fs::read(sequence).unwrap();
        let smf = Smf::parse(&bytes).unwrap();
        let sequence: Rc<midi_player::MidiSequence> = Rc::new(smf.into());
        let mut midi_players: Vec<MidiPlayer> = vec![];

        if let Ok(vst_host) = vstd.lock() {
            for d in &vst_host.devices {
                let midi_player = MidiPlayer::new(
                    sequence.clone(),
                    SAMPLE_RATE as f32,
                    FRAME_SIZE,
                    d.device_filter,
                );

                midi_players.push(midi_player);
            }
        }

        println!("Starting MIDI player!");

        let logger = MidiLogger::new(MidiLogFilter {
            note_on: args.note_on_log,
            note_off: args.note_off_log,
            ctrl: args.controller_log,
            pitch: args.pitch_log,
            loops: args.loops_log,
        });

        loop {
            let device_events = midi_players
                .iter_mut()
                .map(|mp| mp.get_next_events())
                .collect::<Vec<_>>();

            if let Ok(mut vst_host) = vstd.lock() {
                vst_host.process_audio(tx.clone(), &device_events);
                for (i, events) in device_events.iter().enumerate() {
                    for evt in events {
                        logger.log_midi_event(&vst_host.devices[i], evt);
                    }
                }
            }
        }
    });

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } => {
                if window_id == main_window.id() {
                    *control_flow = ControlFlow::Exit
                }
                if let Ok(mut vst_host) = vste.lock() {
                    for (i, window) in windows.iter_mut().enumerate() {
                        if window.id() == window_id {
                            let d = &mut vst_host.devices[i];
                            if let Some(mut e) = d.plugin.get_editor() {
                                e.close();
                                window.set_visible(false);
                            }
                        }
                    }
                }
            }

            _ => (),
        }
    });
}
