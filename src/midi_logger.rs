use sample_host::Device;
use vst::event::MidiEvent;

use crate::sample_host;

pub fn midi_note_to_name_octave(note_number: u8) -> (String, i32) {
    let note_name = match note_number % 12 {
        0 => "C ",
        1 => "C♯",
        2 => "D ",
        3 => "D♯",
        4 => "E ",
        5 => "F ",
        6 => "F♯",
        7 => "G ",
        8 => "G♯",
        9 => "A ",
        10 => "A♯",
        11 => "B ",
        _ => "unknown",
    };
    let octave = ((note_number / 12) as i32) - 2;
    (note_name.to_owned(), octave)
}

pub struct MidiLogFilter {
    pub note_on: bool,
    pub note_off: bool,
    pub ctrl: bool,
    pub pitch: bool,
    pub loops: bool,
}

pub struct MidiLogger {
    filter: MidiLogFilter,
}

impl MidiLogger {
    pub fn new(filter: MidiLogFilter) -> Self {
        Self { filter }
    }
    pub fn log_midi_event(&self, device: &Device, evt: &MidiEvent) {
        let channel = evt.data[0] & 0xF;

        let status_type = match evt.data[0] / 0x10 {
            8 => "note_off",
            9 => {
                if evt.data[2] == 0 {
                    "note_off"
                } else {
                    "note"
                }
            }
            0xB => "ctrl",
            0xE => "pitch",
            _ => "unknown",
        };

        match status_type {
            "note" => {
                if !self.filter.note_on {
                    return;
                }
                let (note_name, octave) = midi_note_to_name_octave(evt.data[1]);
                println!(
                    "ch {}\t{}\t{} {}\tvel {}\t\tdevice {}\t{}",
                    channel,
                    status_type,
                    note_name,
                    octave,
                    evt.data[2],
                    device.device_filter,
                    device.name
                );
            }
            "note_off" => {
                if !self.filter.note_off {
                    return;
                }
                let (note_name, octave) = midi_note_to_name_octave(evt.data[1]);
                println!(
                    "ch {}\t{}\t{} {}\tvel {}\t\tdevice {}\t{}",
                    channel,
                    "noff",
                    note_name,
                    octave,
                    evt.data[2],
                    device.device_filter,
                    device.name
                );
            }
            "ctrl" => match evt.data[1] {
                112 => {
                    if !self.filter.loops {
                        return;
                    }
                    println!("--------------------------- LOOP START ---------------------------");
                }
                113 => {
                    if !self.filter.loops {
                        return;
                    }
                    println!("---------------------------- LOOP END ----------------------------");
                }
                _ => {
                    if !self.filter.ctrl {
                        return;
                    }
                    println!(
                        "ch {}\t{}\tnum {}\tval {}\t\tdevice {}\t{}",
                        channel,
                        status_type,
                        evt.data[1],
                        evt.data[2],
                        device.device_filter,
                        device.name
                    );
                }
            },
            "pitch" => {
                if !self.filter.pitch {
                    return;
                }

                let pitch = ((((evt.data[1] as u32) + ((evt.data[2] as u32) << 7)) as f32)
                    - 0x2000 as f32)
                    / (0x2000 as f32)
                    * 2.0;
                println!(
                    "ch {}\t{}\t\t{:.2}\t\tdevice {}\t{}",
                    channel, status_type, pitch, device.device_filter, device.name
                );
            }
            _ => (),
        }
    }
}
