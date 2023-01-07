use vst::event::MidiEvent;

pub struct TimedEvent {
    /* we don't have a tick system yet, so we're just snapping to frame */
    pub frames: usize,
    pub event: MidiEvent,
}

impl TimedEvent {
    pub fn new(frames: usize, event: MidiEvent) -> Self {
        TimedEvent { frames, event }
    }
}

pub enum SequenceType {
    Absolute,
    Relative,
}

pub struct EventSequence {
    pub events: Vec<TimedEvent>,
    pub sequence_type: SequenceType,
}

impl EventSequence {
    pub fn new(events: Vec<TimedEvent>, sequence_type: SequenceType) -> Self {
        EventSequence {
            events,
            sequence_type,
        }
    }
}

pub struct TimedEventPlayer {
    sequence: EventSequence,
    event_index: usize,
    current_frame: usize,
    next_frame: usize,
}

impl TimedEventPlayer {
    pub fn new(sequence: EventSequence) -> Self {
        TimedEventPlayer {
            sequence,
            event_index: 0,
            current_frame: 0,
            next_frame: 0,
        }
    }

    pub fn play(&mut self, delta: usize, event_buffer: &mut Vec<MidiEvent>) {
        event_buffer.clear();
        self.current_frame += delta;

        while self.event_index < self.sequence.events.len() {
            let seq_event = &self.sequence.events[self.event_index];
            let should_fire = match self.sequence.sequence_type {
                SequenceType::Absolute => self.current_frame >= seq_event.frames,
                SequenceType::Relative => self.current_frame >= self.next_frame,
            };

            if !should_fire {
                break;
            }

            event_buffer.push(seq_event.event);
            if let SequenceType::Relative = self.sequence.sequence_type {
                self.next_frame = self.current_frame + seq_event.frames;
            }
            self.event_index += 1;
        }
    }
}
