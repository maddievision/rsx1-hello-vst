use vst::event::MidiEvent;

pub fn log_midi_event(device_id: usize, evt: &MidiEvent) {
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
                device_id, channel, status_type, note_num, octave, evt.data[2]
            );
        }
        "control" => {
            println!(
                "device {} - sending MIDI: ch {}\t{}\tnum {}\tval {}",
                device_id, channel, status_type, evt.data[1], evt.data[2]
            );
        }
        "pitch" => {
            let pitch = ((((evt.data[1] as u32) + ((evt.data[2] as u32) << 7)) as f32)
                - 0x2000 as f32)
                / (0x2000 as f32)
                * 2.0;
            println!(
                "device {} - sending MIDI: ch {}\t{}\t{:.2}",
                device_id, channel, status_type, pitch,
            );
        }
        _ => (),
    }
}
