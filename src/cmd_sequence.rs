pub use midi_types::Note;
use vst::event::MidiEvent;

use crate::timed_event::{EventSequence, SequenceType, TimedEvent};

pub struct NoteEvent {
    pub channel: u8,
    pub note: Note,
    pub velocity: u8,
}

impl NoteEvent {
    pub fn new(channel: u8, note: Note, velocity: u8) -> Self {
        NoteEvent {
            channel,
            note,
            velocity,
        }
    }
}

impl From<&NoteEvent> for MidiEvent {
    fn from(note_event: &NoteEvent) -> Self {
        MidiEvent {
            data: [
                0x90 + note_event.channel,
                note_event.note.into(),
                note_event.velocity,
            ],
            delta_frames: 0,
            live: true,
            note_length: None,
            note_offset: None,
            detune: 0,
            note_off_velocity: 0,
        }
    }
}

pub enum Cmd {
    WaitCmd,
    NoteCmd(NoteEvent),
    MidiStatusCmd(MidiEvent),
    RepeatCmd(i32),
    RepeatEndCmd,
}

pub struct TimedCmdEvent {
    /* we don't have a tick system yet, so we're just snapping to frame */
    pub frames: usize,
    pub cmd: Cmd,
}

impl TimedCmdEvent {
    pub fn new(frames: usize, cmd: Cmd) -> Self {
        TimedCmdEvent { frames, cmd }
    }
}

pub struct CmdSequence {
    pub frame_start: usize,
    pub events: Vec<TimedCmdEvent>,
}

impl CmdSequence {
    pub fn new(frame_start: usize, events: Vec<TimedCmdEvent>) -> Self {
        CmdSequence {
            frame_start,
            events,
        }
    }
}

impl From<CmdSequence> for Vec<TimedEvent> {
    fn from(sequence: CmdSequence) -> Self {
        let mut out: Vec<TimedEvent> = vec![];

        let mut i = 0;
        let mut frames = sequence.frame_start;
        let mut rpt_count = 0;
        let mut rpt_start: usize = 0;
        while i < sequence.events.len() {
            let events = &sequence.events;
            let event = &events[i];
            match &event.cmd {
                Cmd::NoteCmd(note_event) => {
                    let midi_event: MidiEvent = note_event.into();
                    out.push(TimedEvent::new(frames, midi_event));
                    i += 1;
                }
                Cmd::MidiStatusCmd(midi_event) => {
                    out.push(TimedEvent::new(frames, *midi_event));
                    i += 1;
                }
                Cmd::RepeatCmd(count) => {
                    rpt_count = *count - 1;
                    i += 1;
                    rpt_start = i;
                }
                Cmd::RepeatEndCmd => {
                    if rpt_count > 0 {
                        rpt_count -= 1;
                        i = rpt_start;
                    } else {
                        i += 1
                    }
                }
                Cmd::WaitCmd => i += 1,
            }
            frames += event.frames;
        }
        out
    }
}

pub struct Playlist {
    pub sequences: Vec<CmdSequence>,
}

impl Playlist {
    pub fn new(sequences: Vec<CmdSequence>) -> Self {
        Playlist { sequences }
    }
}

impl From<Playlist> for EventSequence {
    fn from(playlist: Playlist) -> Self {
        let mut out: Vec<TimedEvent> = vec![];
        for sequence in playlist.sequences {
            let mut evts: Vec<TimedEvent> = sequence.into();
            out.append(&mut evts);
        }
        out.sort_by(|a, b| a.frames.cmp(&b.frames));
        EventSequence::new(out, SequenceType::Absolute)
    }
}
