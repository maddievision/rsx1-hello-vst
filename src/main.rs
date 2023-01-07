use crossbeam::channel::bounded;
use sample_host::VstHost;
use std::thread;
use vst::event::MidiEvent;
use vst::host::HostBuffer;

use crate::timed_event::TimedEventPlayer;

extern crate vst;

pub mod audio_host;
pub mod cmd_sequence;
pub mod sample_host;
pub mod schala;
pub mod timed_event;
fn main() {
    let (tx, rx) = bounded::<Vec<f32>>(8);

    let _stream = audio_host::start(rx.clone(), 48000, 256);

    thread::spawn(move || {
        let mut vst_host = VstHost::new(48000.0);
        let mut first_frame = true;
        let mut host_buffer: HostBuffer<f32> = HostBuffer::new(0, 2);
        let inputs = vec![vec![0.0; 256]; 0];
        let mut outputs = vec![vec![0.0; 256]; 2];
        let mut audio_buffer = host_buffer.bind(&inputs, &mut outputs);
        const STEP: usize = 16;
        let mut event_player = TimedEventPlayer::new(schala::build_schala(STEP));
        println!("Starting event player!");
        loop {
            let mut events: Vec<MidiEvent> = Vec::with_capacity(256);
            match first_frame {
                true => event_player.play(0, &mut events),
                false => event_player.play(1, &mut events),
            }

            first_frame = false;
            for evt in events.as_slice() {
                let status_type = match evt.data[0] / 0x10 {
                    8 => "note_off",
                    9 => {
                        if evt.data[2] == 0 {
                            "note_off"
                        } else {
                            "note_on "
                        }
                    }
                    _ => "unknown",
                };
                let channel = evt.data[0] & 0xF;
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
                    "sending MIDI: ch {}\t{}\t{}-{}\tvel {}",
                    channel, status_type, note_num, octave, evt.data[2]
                );
            }

            vst_host.process_audio(tx.clone(), &mut audio_buffer, events);
        }
    });

    loop {}
}
