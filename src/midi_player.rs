const EVENT_BUFFER_SIZE: usize = 512;

use midly::{MetaMessage, MidiMessage, PitchBend, Smf, TrackEvent, TrackEventKind};
use std::rc::Rc;
use vst::event::MidiEvent;

#[derive(Debug)]
pub struct MidiTimedEvent<'a> {
    ts_ticks: usize,
    device: usize,
    event: TrackEvent<'a>,
}

pub struct MidiSequence<'a> {
    ppqn: usize,
    events: Vec<MidiTimedEvent<'a>>,
}

impl<'a> From<Smf<'a>> for MidiSequence<'a> {
    fn from(smf: Smf<'a>) -> Self {
        let mut all_events: Vec<MidiTimedEvent> = vec![];
        let ppqn = match smf.header.timing {
            midly::Timing::Metrical(ppqn) => u16::from(ppqn) as usize,
            _ => 960,
        };

        for track in smf.tracks {
            let mut device = 0;
            let mut ts_ticks = 0;
            for event in track {
                ts_ticks += u32::from(event.delta) as usize;
                if let TrackEventKind::Midi {
                    channel: _,
                    message,
                } = event.kind
                {
                    if let MidiMessage::Controller { controller, value } = message {
                        if controller == 111 {
                            device = u8::from(value) as usize;
                        }
                    }
                }
                all_events.push(MidiTimedEvent {
                    ts_ticks,
                    device,
                    event,
                })
            }
        }

        all_events.sort_by(|a, b| a.ts_ticks.cmp(&b.ts_ticks));
        MidiSequence {
            ppqn,
            events: all_events,
        }
    }
}

pub struct MidiPlayer<'a> {
    sequence: Rc<MidiSequence<'a>>,
    frame_size: usize,
    frame_counter: usize,
    sample_rate: f32,
    event_index: usize,
    tempo: f32,
    last_event_tick: usize,
    last_event_ms: f32,
    device: usize,
    loop_index: Option<usize>,
}

impl<'a> MidiPlayer<'a> {
    pub fn new(
        sequence: Rc<MidiSequence<'a>>,
        sample_rate: f32,
        frame_size: usize,
        device: usize,
    ) -> Self {
        Self {
            sequence,
            sample_rate,
            frame_size,
            frame_counter: 0,
            event_index: 0,
            tempo: 120.0,
            last_event_tick: 0,
            last_event_ms: 0.0,
            device,
            loop_index: None,
        }
    }

    pub fn get_next_events(&mut self) -> Vec<MidiEvent> {
        let mut event_buffer: Vec<MidiEvent> = Vec::with_capacity(EVENT_BUFFER_SIZE);

        let frame_len_ms = (self.frame_size * 1000) as f32 / self.sample_rate;
        let frame_start_ms = frame_len_ms * self.frame_counter as f32;
        let mspt = (60000.0 / self.tempo) / self.sequence.ppqn as f32;

        while self.event_index < self.sequence.events.len() {
            let evt = &self.sequence.events[self.event_index];
            let ticks_since_last = evt.ts_ticks - self.last_event_tick;
            let ms_since_last = ticks_since_last as f32 * mspt;
            let target_ms = self.last_event_ms + ms_since_last;
            let target_frame = ((target_ms - frame_start_ms) / 1000.0) * self.sample_rate;
            let target_frame = target_frame.clamp(0.0, target_frame) as usize;

            if target_frame >= self.frame_size {
                // exit when target past the frame boundary
                break;
            }

            self.last_event_tick = evt.ts_ticks;
            self.last_event_ms = target_ms;
            self.event_index += 1;

            if let TrackEventKind::Meta(message) = evt.event.kind {
                if let MetaMessage::Tempo(uspt) = message {
                    self.tempo = 60000000.00 / u32::from(uspt) as f32;
                }
            }

            // special meta event-like controllers
            if let TrackEventKind::Midi { channel, message } = evt.event.kind {
                if let MidiMessage::Controller { controller, value } = message {
                    match u8::from(controller) {
                        112 => {
                            // loop start
                            self.loop_index = Some(self.event_index);
                        }
                        113 => {
                            // loop end
                            if let Some(idx) = self.loop_index {
                                self.event_index = idx;
                                let levt = &self.sequence.events[idx];
                                self.last_event_tick = levt.ts_ticks;
                                event_buffer.push(MidiEvent {
                                    data: [
                                        0xB0 + u8::from(channel),
                                        u8::from(controller),
                                        u8::from(value),
                                    ],
                                    delta_frames: target_frame as i32,
                                    live: false,
                                    note_length: None,
                                    note_offset: None,
                                    detune: 0,
                                    note_off_velocity: 0,
                                });
                                continue;
                            }
                        }
                        _ => (),
                    }
                }
            }

            if evt.device != self.device {
                continue;
            }

            let midi_event: Option<MidiEvent> = match evt.event.kind {
                TrackEventKind::Midi { channel, message } => match message {
                    MidiMessage::NoteOff { key, vel } => Some(MidiEvent {
                        data: [0x80 + u8::from(channel), u8::from(key), u8::from(vel)],
                        delta_frames: target_frame as i32,
                        live: false,
                        note_length: None,
                        note_offset: None,
                        detune: 0,
                        note_off_velocity: 0,
                    }),
                    MidiMessage::NoteOn { key, vel } => Some(MidiEvent {
                        data: [0x90 + u8::from(channel), u8::from(key), u8::from(vel)],
                        delta_frames: target_frame as i32,
                        live: false,
                        note_length: None,
                        note_offset: None,
                        detune: 0,
                        note_off_velocity: 0,
                    }),
                    MidiMessage::Controller { controller, value } => Some(MidiEvent {
                        data: [
                            0xB0 + u8::from(channel),
                            u8::from(controller),
                            u8::from(value),
                        ],
                        delta_frames: target_frame as i32,
                        live: false,
                        note_length: None,
                        note_offset: None,
                        detune: 0,
                        note_off_velocity: 0,
                    }),
                    MidiMessage::PitchBend { bend } => match bend {
                        PitchBend(b) => Some(MidiEvent {
                            data: [
                                0xE0 + u8::from(channel),
                                (u16::from(b) & 0x7F) as u8,
                                (u16::from(b) >> 7) as u8,
                            ],
                            delta_frames: target_frame as i32,
                            live: false,
                            note_length: None,
                            note_offset: None,
                            detune: 0,
                            note_off_velocity: 0,
                        }),
                    },
                    _ => None,
                },
                _ => None,
            };

            if let Some(e) = midi_event {
                event_buffer.push(e);
            }
        }
        self.frame_counter += 1;
        event_buffer
    }
}
