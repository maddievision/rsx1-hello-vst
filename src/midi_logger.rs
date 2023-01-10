use sample_host::Device;
use vst::event::MidiEvent;

use crate::sample_host;

pub fn log_midi_event(device: &Device, evt: &MidiEvent) {
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
            let note_num = match evt.data[1] % 12 {
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
            let octave = ((evt.data[1] / 12) as i32) - 2;
            println!(
                "ch {}\t{}\t{} {}\tvel {}\t\tdevice {}\t{}",
                channel,
                status_type,
                note_num,
                octave,
                evt.data[2],
                device.device_filter,
                device.name
            );
        }
        "ctrl" => match evt.data[1] {
            112 => {
                println!("");
                println!("=================================================================");
                println!("=========================== LOOP START ===========================");
                println!("==================================================================");
                println!("");
            }
            113 => {
                println!("");
                println!("=================================================================");
                println!("=========================== LOOP END  ===========================");
                println!("==================================================================");
                println!("");
            }
            _ => {
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
