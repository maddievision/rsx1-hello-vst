use crossbeam::channel::bounded;
use sample_host::VstHost;
use std::rc::Rc;
use std::thread;
use vst::host::HostBuffer;
use crate::midi_player::MidiPlayer;
use midly::Smf;
use std::fs;

extern crate vst;

pub mod audio_host;
pub mod midi_player;
pub mod sample_host;

fn main() {
    // Load bytes into a buffer

    let (tx, rx) = bounded::<Vec<f32>>(8);
    let _stream = audio_host::start(rx.clone(), 48000, 1024);

    thread::spawn(move || {
        let bytes = fs::read("data/schala.mid").unwrap();
        let smf = Smf::parse(&bytes).unwrap();
        let sequence: Rc<midi_player::MidiSequence> = Rc::new(smf.into());
        let mut vst_host = VstHost::new(48000.0);
        vst_host.create_device(
            "/Library/Audio/Plug-Ins/VST/chipsynth SFC.vst/Contents/MacOS/chipsynth SFC"
                .to_string(),
            "data/schala_inst.fxp".to_string(),
        );
        vst_host.create_device(
            "/Library/Audio/Plug-Ins/VST/chipsynth SFC.vst/Contents/MacOS/chipsynth SFC"
                .to_string(),
            "data/schala_perc.fxp".to_string(),
        );

        // let mut first_frame = true;
        let mut host_buffer: HostBuffer<f32> = HostBuffer::new(0, 2);
        let inputs = vec![vec![0.0; 1024]; 0];
        let mut outputs = vec![vec![0.0; 1024]; 2];
        let mut audio_buffer = host_buffer.bind(&inputs, &mut outputs);
        const STEP: usize = 16;
        let mut midi_players = vec![
            MidiPlayer::new(sequence.clone(), 48000.0, 1024, 0),
            MidiPlayer::new(sequence.clone(), 48000.0, 1024, 1),
        ];

        println!("Starting MIDI player!");

        loop {
            let device_events = midi_players
                .iter_mut()
                .map(|mp| mp.get_next_events())
                .collect::<Vec<_>>();

            for (i, events) in device_events.iter().enumerate() {
                for evt in events {
                    let channel = evt.data[0] & 0xF;

                    let status_type = match evt.data[0] / 0x10 {
                        8 => "note_off",
                        9 => {
                            if evt.data[2] == 0 {
                                "note_off"
                            } else {
                                "note_on"
                            }
                        }
                        0xB => "control",
                        0xE => "pitch",
                        _ => "unknown",
                    };

                    match status_type {
                        "note_on" => {
                            let note_num = match evt.data[1] % 12 {
                                0 => "C",
                                1 => "C♯",
                                2 => "D",
                                3 => "D♯",
                                4 => "E",
                                5 => "F",
                                6 => "F♯",
                                7 => "G",
                                8 => "G♯",
                                9 => "A",
                                10 => "A♯",
                                11 => "B",
                                _ => "unknown",
                            };
                            let octave = ((evt.data[1] / 12) as i32) - 2;
                            println!(
                                "device {} - sending MIDI: ch {}\t{}\t{}-{}\tvel {}",
                                i, channel, status_type, note_num, octave, evt.data[2]
                            );
                        }
                        "control" => {
                            println!(
                                "device {} - sending MIDI: ch {}\t{}\tnum {}\tval {}",
                                i, channel, status_type, evt.data[1], evt.data[2]
                            );
                        }
                        "pitch" => {
                            let pitch = ((((evt.data[1] as u32) + ((evt.data[2] as u32) << 7))
                                as f32)
                                - 0x2000 as f32)
                                / (0x2000 as f32)
                                * 2.0;
                            println!(
                                "device {} - sending MIDI: ch {}\t{}\t{:.2}",
                                i, channel, status_type, pitch,
                            );
                        }
                        _ => (),
                    }
                }
            }

            vst_host.process_audio(tx.clone(), &mut audio_buffer, device_events);
        }
    });

    loop {}
}
