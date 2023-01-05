use midi_types::Note;
use vst::event::MidiEvent;

pub struct TimedEvent {
    /* we don't have a tick system yet, so we're just snapping to frame */
    pub frame_len: usize,
    pub event: MidiEvent,
}

impl TimedEvent {
    fn new(frame_len: usize, event: MidiEvent) -> Self {
        TimedEvent {
            frame_len: frame_len,
            event: event,
        }
    }
}

pub fn build_timed_note(frame_len: usize, channel: u8, note: Note, velocity: u8) -> TimedEvent {
    TimedEvent::new(
        frame_len,
        MidiEvent {
            data: [0x90 + channel, note.into(), velocity],
            delta_frames: 0,
            live: true,
            note_length: None,
            note_offset: None,
            detune: 0,
            note_off_velocity: 0,
        },
    )
}
